from concurrent.futures import ThreadPoolExecutor
import hashlib
import io
import json
from pathlib import Path
import struct
import sys
import tempfile
import unittest
from unittest.mock import patch
import zlib

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import gallery_metadata as metadata


def png_chunk(kind, body):
    return struct.pack(">I", len(body)) + kind + body + struct.pack(">I", zlib.crc32(kind + body) & 0xffffffff)


def png(color="red", extra=b""):
    from PIL import Image
    stream = io.BytesIO()
    Image.new("RGB", (16, 8), color).save(stream, format="PNG")
    body = stream.getvalue()
    return body[:-12] + extra + body[-12:]


class Response(io.BytesIO):
    status = 200


class GalleryCacheTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        (self.root / "shadow.toml").write_text('version = 1\nname = "test-site"\n[backend]\ntype = "volcengine_tos"\nendpoint = "https://tos.example.test"\nbucket = "photos"\n')
        self.objects = {}
        self.calls = []
        self.network = patch.object(metadata, "urlopen", side_effect=self.download)
        self.network.start()
        self.addCleanup(self.network.stop)

    def add_ref(self, name, body):
        digest = hashlib.sha256(body).hexdigest()
        oid = "sha256:" + digest
        ref = self.root / ".shadow/refs/gallery" / (name + ".ref")
        ref.parent.mkdir(parents=True, exist_ok=True)
        ref.write_text(f'version = 1\noid = "{oid}"\nsize = {len(body)}\n')
        self.objects[oid] = body
        return oid

    def download(self, request, **kwargs):
        self.calls.append(request.full_url)
        self.assertIsNone(request.get_header("Range"))
        digest = "".join(request.full_url.split("/")[-2:])
        return Response(self.objects["sha256:" + digest])

    def sync(self, **kwargs):
        return metadata.ensure_metadata(self.root, **kwargs)

    def test_cold_build_creates_single_hash_indexed_cache(self):
        oid = self.add_ref("album/photo.png", png())
        result = self.sync()
        self.assertEqual(result["version"], 2)
        self.assertEqual(result["extractor_version"], metadata.EXTRACTOR_VERSION)
        self.assertEqual(list(result["objects"]), [oid])
        self.assertEqual((result["objects"][oid]["width"], result["objects"][oid]["height"]), (16, 8))
        self.assertNotIn("path", result["objects"][oid])
        self.assertNotIn("created", result["objects"][oid])  # No filename/time fallback baked into cache.
        self.assertEqual(len(self.calls), 1)
        self.assertEqual(len(list((self.root / "target/gallery-metadata").glob("*.json"))), 1)
        self.assertEqual(list((self.root / "target").iterdir()), [self.root / "target/gallery-metadata"])
        self.assertFalse((self.root / "gallery").exists())

    def test_warm_build_is_offline_and_does_not_rewrite_cache(self):
        self.add_ref("album/photo.png", png())
        expected = self.sync()
        before = metadata.cache_path(self.root).stat().st_mtime_ns
        with patch.object(metadata, "urlopen", side_effect=AssertionError("warm build contacted network")):
            self.assertEqual(self.sync(), expected)
        self.assertEqual(metadata.cache_path(self.root).stat().st_mtime_ns, before)

    def test_adding_one_image_fetches_only_that_object(self):
        self.add_ref("album/a.png", png())
        self.sync()
        self.calls.clear()
        self.add_ref("album/b.png", png("blue"))
        result = self.sync()
        self.assertEqual(len(result["objects"]), 2)
        self.assertEqual(len(self.calls), 1)

    def test_duplicates_and_renames_reuse_content_hash(self):
        body = png()
        self.add_ref("album/a.png", body)
        self.add_ref("other/a.png", body)
        self.sync()
        self.assertEqual(len(self.calls), 1)
        ref = self.root / ".shadow/refs/gallery/album/a.png.ref"
        ref.rename(ref.with_name("renamed.png.ref"))
        self.calls.clear()
        self.sync()
        self.assertEqual(self.calls, [])

    def test_replaced_image_fetches_new_hash_and_keeps_reusable_old_entry(self):
        before = self.add_ref("album/a.png", png())
        self.sync()
        after = self.add_ref("album/a.png", png("blue"))
        self.calls.clear()
        result = self.sync()
        self.assertNotEqual(before, after)
        self.assertEqual(set(result["objects"]), {before, after})
        self.assertEqual(len(self.calls), 1)

    def test_only_corrupt_entry_is_repaired(self):
        first = self.add_ref("album/a.png", png())
        self.add_ref("album/b.png", png("blue"))
        result = self.sync()
        result["objects"][first]["width"] = 0
        metadata.write_cache(metadata.cache_path(self.root), result)
        self.calls.clear()
        self.assertEqual(self.sync()["objects"][first]["width"], 16)
        self.assertEqual(len(self.calls), 1)

    def test_invalid_entry_field_types_are_cache_misses(self):
        oid = self.add_ref("album/a.png", png())
        result = self.sync()
        entry = result["objects"][oid]
        record = metadata.references(self.root)[0]
        for changes in [{"width": True}, {"size": "1"}, {"rating": "5"}, {"created": "yesterday"}, {"label": []}]:
            self.assertFalse(metadata.valid_entry({**entry, **changes}, record))

    def test_extractor_version_invalidates_cache(self):
        oid = self.add_ref("album/a.png", png())
        result = self.sync()
        result["extractor_version"] -= 1
        metadata.write_cache(metadata.cache_path(self.root), result)
        self.calls.clear()
        self.assertIn(oid, self.sync()["objects"])
        self.assertEqual(len(self.calls), 1)

    def test_legacy_and_malformed_json_are_regenerated(self):
        self.add_ref("album/a.png", png())
        path = metadata.cache_path(self.root)
        path.parent.mkdir(parents=True)
        for value in ['{"version":1,"images":{}}', 'not JSON', '[]']:
            path.write_text(value)
            self.calls.clear()
            self.assertEqual(self.sync()["version"], 2)
            self.assertEqual(len(self.calls), 1)

    def test_local_original_is_used_and_hash_checked(self):
        body = png()
        self.add_ref("album/a.png", body)
        original = self.root / "gallery/album/a.png"
        original.parent.mkdir(parents=True)
        original.write_bytes(body)
        self.sync()
        self.assertEqual(self.calls, [])
        original.write_bytes(png("blue"))
        with self.assertRaisesRegex(RuntimeError, "SHA-256|exceeds"):
            self.sync(refresh=True)
        self.assertEqual(original.read_bytes(), png("blue"))

    def test_duplicate_hash_prefers_any_available_local_original(self):
        body = png()
        self.add_ref("album/a.png", body)
        self.add_ref("album/z.png", body)
        original = self.root / "gallery/album/z.png"
        original.parent.mkdir(parents=True)
        original.write_bytes(body)
        self.sync()
        self.assertEqual(self.calls, [])

    def test_failed_sibling_retains_successes_for_next_build(self):
        good = self.add_ref("album/a.png", png())
        bad = self.add_ref("album/b.png", png("blue"))
        correct = self.objects[bad]
        self.objects[bad] = b"invalid object"
        with self.assertRaisesRegex(RuntimeError, "extraction failed"):
            self.sync()
        data = metadata.read_cache(metadata.cache_path(self.root))
        self.assertIn(good, data["objects"])
        self.assertNotIn(bad, data["objects"])
        self.objects[bad] = correct
        self.calls.clear()
        self.sync()
        self.assertEqual(len(self.calls), 1)

    def test_wrong_digest_or_size_is_not_cached(self):
        oid = self.add_ref("album/a.png", png())
        original = self.objects[oid]
        for wrong in [original[:-1], original + b"extra", bytes([original[0] ^ 1]) + original[1:]]:
            self.objects[oid] = wrong
            with self.assertRaises(RuntimeError):
                self.sync()
            self.assertNotIn(oid, metadata.read_cache(metadata.cache_path(self.root))["objects"])

    def test_partial_http_response_is_rejected(self):
        self.add_ref("album/a.png", png())
        with patch.object(Response, "status", 206), self.assertRaisesRegex(RuntimeError, "complete object"):
            self.sync()

    def test_conflicting_ref_sizes_fail_before_network(self):
        body = png()
        self.add_ref("album/a.png", body)
        self.add_ref("album/b.png", body)
        records = metadata.references(self.root)
        records[-1]["size"] += 1
        with self.assertRaisesRegex(ValueError, "conflicting Shadow sizes"):
            self.sync(records=records)
        self.assertEqual(self.calls, [])

    def test_simultaneous_builds_do_not_duplicate_downloads(self):
        self.add_ref("album/a.png", png())
        with ThreadPoolExecutor(max_workers=2) as executor:
            results = list(executor.map(lambda _: self.sync(), range(2)))
        self.assertEqual(results[0], results[1])
        self.assertEqual(len(self.calls), 1)

    def test_png_metadata_in_middle_of_large_object_is_found(self):
        xmp = b'<x:xmpmeta xmlns:x="adobe:ns:meta/"><r xmlns:xmp="http://ns.adobe.com/xap/1.0/" xmp:Rating="5" xmp:Label="Red"/></x:xmpmeta>'
        padding = png_chunk(b"vpAg", b"\0" * 300000)
        body = png(extra=padding + png_chunk(b"tEXt", b"XML:com.adobe.xmp\0" + xmp) + padding)
        oid = self.add_ref("album/a.png", body)
        entry = self.sync()["objects"][oid]
        self.assertEqual((entry["rating"], entry["label"]), (5, "Red"))

    def test_compressed_png_xmp_is_parsed(self):
        xmp = b'<x:xmpmeta xmlns:x="adobe:ns:meta/"><r xmlns:xmp="http://ns.adobe.com/xap/1.0/" xmp:Rating="4"/></x:xmpmeta>'
        for kind, payload in [(b"zTXt", b"XML:com.adobe.xmp\0\0" + zlib.compress(xmp)),
                              (b"iTXt", b"XML:com.adobe.xmp\0\x01\x00\0\0" + zlib.compress(xmp))]:
            with self.subTest(kind=kind):
                fields = metadata.png_fields(png(extra=png_chunk(kind, payload)))
                self.assertEqual(fields["rating"], 4)

    def test_png_crc_and_truncation_are_errors(self):
        for value in [png()[:-2], png()[:29] + b"bad!" + png()[33:]]:
            with self.assertRaisesRegex(ValueError, "truncated|CRC"):
                metadata.png_fields(value)

    def test_metadata_decompression_is_bounded(self):
        with patch.object(metadata, "MAX_METADATA_BYTES", 1024), self.assertRaisesRegex(ValueError, "oversized"):
            metadata.decompress_metadata(zlib.compress(b"x" * 2048))

    def test_streaming_reads_are_bounded(self):
        class Bounded(io.BytesIO):
            def read(inner, size=-1):
                self.assertGreater(size, 0)
                self.assertLessEqual(size, metadata.CHUNK_SIZE)
                return super().read(size)
        body = b"x" * (metadata.CHUNK_SIZE * 2 + 7)
        record = {"size": len(body), "oid": "sha256:" + hashlib.sha256(body).hexdigest()}
        target = io.BytesIO()
        metadata.verified_copy(Bounded(body), target, record)
        self.assertEqual(target.read(), body)

    def test_atomic_write_keeps_previous_file_if_replace_fails(self):
        self.add_ref("album/a.png", png())
        self.sync()
        path = metadata.cache_path(self.root)
        previous = path.read_bytes()
        with patch.object(Path, "replace", side_effect=OSError("interrupted")), self.assertRaises(OSError):
            metadata.write_cache(path, metadata.empty_cache())
        self.assertEqual(path.read_bytes(), previous)
        self.assertFalse(list(path.parent.glob("*.tmp")))


if __name__ == "__main__":
    unittest.main()
