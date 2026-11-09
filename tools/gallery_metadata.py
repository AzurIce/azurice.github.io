"""Incremental, content-addressed metadata cache for Shadow gallery objects.

One generated JSON lives under target/, never in site/data or Git. Missing
objects are streamed to temporary files, SHA-256 checked, fully scanned for
JPEG/PNG metadata and discarded. Warm builds do not contact object storage.
"""

import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
from contextlib import contextmanager
from datetime import datetime
import hashlib
import io
import json
import os
from pathlib import Path
import re
import struct
import sys
import tempfile
import time
import tomllib
from urllib.parse import quote, urlsplit
from urllib.request import Request, urlopen
import xml.etree.ElementTree as ET
import zlib

ROOT = Path(__file__).resolve().parents[1]
CACHE_FORMAT_VERSION = 2
EXTRACTOR_VERSION = 3
MAX_METADATA_BYTES = 16 * 1024 * 1024
CHUNK_SIZE = 1024 * 1024


def cache_path(root=ROOT):
    return root / "target/gallery-metadata/metadata.json"


def empty_cache():
    return {"version": CACHE_FORMAT_VERSION, "extractor_version": EXTRACTOR_VERSION, "objects": {}}


def references(root=ROOT):
    config = tomllib.loads((root / "shadow.toml").read_text())
    backend = config["backend"]
    if backend["type"] != "volcengine_tos":
        raise ValueError("gallery requires volcengine_tos")
    endpoint = urlsplit(backend["endpoint"])
    prefix = "/".join(quote(part, safe="") for part in [*filter(None, backend.get("prefix", "").split("/")), config["name"]])
    base = f"{endpoint.scheme}://{backend['bucket']}.{endpoint.netloc}/{prefix}/objects/sha256"
    result = []
    for path in sorted((root / ".shadow/refs/gallery").rglob("*.ref")):
        record = tomllib.loads(path.read_text())
        if record.get("version") != 1 or not re.fullmatch(r"sha256:[a-fA-F0-9]{64}", record.get("oid", "")):
            raise ValueError(f"invalid object ID: {path}")
        if type(record.get("size")) is not int or record["size"] <= 0:
            raise ValueError(f"invalid object size: {path}")
        digest = record["oid"][7:].lower()
        relative = path.relative_to(root / ".shadow/refs/gallery").as_posix().removesuffix(".ref")
        result.append({"path": relative, "oid": "sha256:" + digest, "size": record["size"],
                       "url": f"{base}/{digest[:2]}/{digest[2:]}"})
    return result


def text(value):
    if isinstance(value, bytes):
        if value.startswith(b"ASCII\0\0\0"):
            value = value[8:]
        elif value.startswith(b"UNICODE\0"):
            return value[8:].decode("utf-16", errors="replace").strip("\0 ")
        return value.decode("utf-8", errors="replace").strip("\0 ")
    return str(value).strip() if value is not None else ""


def exif_fields(payload):
    from PIL import Image
    exif = Image.Exif()
    exif.load(payload if payload.startswith(b"Exif\0\0") else b"Exif\0\0" + payload)
    details = exif.get_ifd(34665) if 34665 in exif else {}
    created = text(details.get(36867) or exif.get(306))
    offset = text(details.get(36881) or details.get(36880))
    if created:
        try:
            normalized = created[:10].replace(":", "-") + created[10:]
            created = datetime.fromisoformat(normalized).isoformat() + offset
        except ValueError:
            created = ""
    return {"title": text(exif.get(270)), "description": text(details.get(37510)), "created": created}


def xmp_fields(payload):
    start = payload.find(b"<x:xmpmeta")
    end = payload.find(b"</x:xmpmeta>")
    if start < 0 or end < start:
        return {}
    node = ET.fromstring(payload[start:end + len(b"</x:xmpmeta>")])
    fields = {}
    namespace = "{http://ns.adobe.com/xap/1.0/}"
    for child in node.iter():
        for name in ("Rating", "Label"):
            value = child.attrib.get(namespace + name)
            if child.tag == namespace + name:
                value = child.text
            if value is not None:
                fields[name.lower()] = int(value) if name == "Rating" else value
    return fields


def read_exact(stream, size):
    value = stream.read(size)
    if len(value) != size:
        raise ValueError("truncated image metadata")
    return value


def decompress_metadata(data):
    inflater = zlib.decompressobj()
    result = inflater.decompress(data, MAX_METADATA_BYTES + 1)
    if len(result) > MAX_METADATA_BYTES or inflater.unconsumed_tail or not inflater.eof:
        raise ValueError("invalid or oversized compressed metadata")
    return result


def png_stream_fields(stream):
    if read_exact(stream, 8) != b"\x89PNG\r\n\x1a\n":
        raise ValueError("invalid PNG signature")
    fields = {}
    while True:
        size, kind = struct.unpack(">I4s", read_exact(stream, 8))
        if kind not in {b"IHDR", b"eXIf", b"iTXt", b"tEXt", b"zTXt", b"IEND"}:
            # IDAT may be hundreds of MB; seeking retains a bounded memory
            # footprint and still finds EXIF/XMP after arbitrary IDAT chunks.
            stream.seek(size + 4, 1)
            continue
        if size > MAX_METADATA_BYTES:
            raise ValueError("oversized PNG metadata chunk")
        body = read_exact(stream, size)
        crc = int.from_bytes(read_exact(stream, 4), "big")
        if zlib.crc32(kind + body) & 0xffffffff != crc:
            raise ValueError("PNG metadata CRC mismatch")
        if kind == b"IHDR":
            if size != 13:
                raise ValueError("invalid PNG IHDR")
            fields["width"], fields["height"] = struct.unpack(">II", body[:8])
        elif kind == b"eXIf":
            fields.update(exif_fields(body))
        elif kind == b"iTXt":
            keyword, rest = body.split(b"\0", 1)
            if keyword == b"XML:com.adobe.xmp":
                compressed = rest[0]
                if compressed not in (0, 1) or rest[1] != 0:
                    raise ValueError("unsupported PNG text compression")
                language, translated, value = rest[2:].split(b"\0", 2)
                fields.update(xmp_fields(decompress_metadata(value) if compressed else value))
        elif kind == b"tEXt":
            keyword, value = body.split(b"\0", 1)
            if keyword == b"XML:com.adobe.xmp":
                fields.update(xmp_fields(value))
        elif kind == b"zTXt":
            keyword, value = body.split(b"\0", 1)
            if keyword == b"XML:com.adobe.xmp":
                if value[0] != 0:
                    raise ValueError("unsupported PNG text compression")
                fields.update(xmp_fields(decompress_metadata(value[1:])))
        elif kind == b"IEND":
            if size != 0 or not fields.get("width") or not fields.get("height"):
                raise ValueError("invalid PNG end/dimensions")
            break
    return fields


def png_fields(data):
    return png_stream_fields(io.BytesIO(data))


def jpeg_stream_fields(stream):
    if read_exact(stream, 2) != b"\xff\xd8":
        raise ValueError("invalid JPEG signature")
    fields = {}
    while True:
        if read_exact(stream, 1) != b"\xff":
            raise ValueError("invalid JPEG marker")
        marker = read_exact(stream, 1)[0]
        while marker == 255:
            marker = read_exact(stream, 1)[0]
        if marker in (0xDA, 0xD9):
            break
        if marker == 0x01 or 0xD0 <= marker <= 0xD8:
            continue
        size = int.from_bytes(read_exact(stream, 2), "big")
        if size < 2:
            raise ValueError("invalid JPEG segment length")
        body = read_exact(stream, size - 2)
        if marker == 0xE1 and body.startswith(b"Exif\0\0"):
            fields.update(exif_fields(body))
        elif marker == 0xE1:
            fields.update(xmp_fields(body))
        elif marker in {0xC0, 0xC1, 0xC2, 0xC3, 0xC5, 0xC6, 0xC7, 0xC9, 0xCA, 0xCB, 0xCD, 0xCE, 0xCF}:
            fields["height"], fields["width"] = struct.unpack(">HH", body[1:5])
    if "width" not in fields:
        raise ValueError("missing JPEG dimensions")
    return fields


def jpeg_fields(data):
    return jpeg_stream_fields(io.BytesIO(data))


def verified_copy(source, destination, record):
    digest = hashlib.sha256()
    total = 0
    while True:
        body = source.read(min(CHUNK_SIZE, record["size"] - total + 1))
        if not body:
            break
        total += len(body)
        if total > record["size"]:
            raise ValueError("image exceeds Shadow ref size")
        digest.update(body)
        destination.write(body)
    if total != record["size"] or "sha256:" + digest.hexdigest() != record["oid"]:
        raise ValueError("image size/SHA-256 differs from Shadow ref")
    destination.seek(0)


def extract(record, root=ROOT):
    original = root / "gallery" / record["path"]
    directory = root / "target"
    directory.mkdir(exist_ok=True)
    with tempfile.TemporaryFile(dir=directory) as stream:
        if original.is_file():
            with original.open("rb") as source:
                verified_copy(source, stream, record)
        else:
            request = Request(record["url"], headers={"Accept-Encoding": "identity", "User-Agent": "azurice-gallery-metadata/2"})
            with urlopen(request, timeout=60) as response:
                if response.status != 200:
                    raise ValueError(f"expected complete object, got HTTP {response.status}")
                verified_copy(response, stream, record)
        signature = stream.read(8)
        stream.seek(0)
        if signature.startswith(b"\xff\xd8"):
            fields = jpeg_stream_fields(stream)
            kind = "jpeg"
        elif signature == b"\x89PNG\r\n\x1a\n":
            fields = png_stream_fields(stream)
            kind = "png"
        else:
            raise ValueError(f"unsupported gallery media: {record['path']}")
    entry = {"size": record["size"], "format": kind, **fields}
    if not valid_entry(entry, record):
        raise ValueError(f"invalid image metadata: {record['path']}")
    return entry


def valid_entry(entry, record):
    if not isinstance(entry, dict) or type(entry.get("size")) is not int or entry["size"] != record["size"]:
        return False
    if entry.get("format") not in {"jpeg", "png"}:
        return False
    if any(type(entry.get(key)) is not int or entry[key] <= 0 for key in ("width", "height")):
        return False
    if any(key in entry and not isinstance(entry[key], str) for key in ("title", "description", "created", "label")):
        return False
    if "rating" in entry and (type(entry["rating"]) is not int or not -1 <= entry["rating"] <= 5):
        return False
    if entry.get("created"):
        try:
            datetime.fromisoformat(entry["created"])
        except ValueError:
            return False
    return True


def read_cache(path):
    try:
        data = json.loads(path.read_text())
    except FileNotFoundError:
        return empty_cache()
    except (ValueError, UnicodeError):
        print(f"Discarding malformed gallery metadata cache: {path}", file=sys.stderr)
        return empty_cache()
    if (not isinstance(data, dict) or data.get("version") != CACHE_FORMAT_VERSION
            or data.get("extractor_version") != EXTRACTOR_VERSION or not isinstance(data.get("objects"), dict)):
        return empty_cache()
    return data


def write_cache(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = None
    try:
        with tempfile.NamedTemporaryFile(mode="w", encoding="utf-8", dir=path.parent, prefix="metadata-", suffix=".tmp", delete=False) as output:
            temporary = Path(output.name)
            json.dump(data, output, ensure_ascii=False, sort_keys=True, indent=2)
            output.write("\n")
            output.flush()
            os.fsync(output.fileno())
        temporary.replace(path)
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)


@contextmanager
def cache_lock(path):
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.with_suffix(".lock").open("a+b") as lock:
        if os.name == "nt":
            import msvcrt
            if lock.tell() == 0:
                lock.write(b"\0")
                lock.flush()
            lock.seek(0)
            while True:
                try:
                    msvcrt.locking(lock.fileno(), msvcrt.LK_NBLCK, 1)
                    break
                except OSError:
                    time.sleep(.1)
        else:
            import fcntl
            fcntl.flock(lock.fileno(), fcntl.LOCK_EX)
        try:
            yield
        finally:
            if os.name == "nt":
                lock.seek(0)
                msvcrt.locking(lock.fileno(), msvcrt.LK_UNLCK, 1)
            else:
                fcntl.flock(lock.fileno(), fcntl.LOCK_UN)


def ensure_metadata(root=ROOT, *, records=None, workers=6, refresh=False):
    records = references(root) if records is None else records
    unique = {}
    for record in records:
        previous = unique.get(record["oid"])
        if previous and previous["size"] != record["size"]:
            raise ValueError(f"conflicting Shadow sizes for {record['oid']}")
        if previous is None or (root / "gallery" / record["path"]).is_file():
            unique[record["oid"]] = record
    path = cache_path(root)
    with cache_lock(path):
        data = read_cache(path)
        objects = data["objects"]
        pending = [record for oid, record in unique.items() if refresh or not valid_entry(objects.get(oid), record)]
        if not pending:
            print(f"Gallery metadata: {len(unique)} cached objects; 0 to fetch/parse.", file=sys.stderr)
            return data
        print(f"Gallery metadata: {len(unique) - len(pending)} cached; extracting {len(pending)} missing/stale objects.", file=sys.stderr)
        errors = []
        with ThreadPoolExecutor(max_workers=max(1, min(workers, 12))) as executor:
            tasks = {executor.submit(extract, record, root): record for record in pending}
            for future in as_completed(tasks):
                record = tasks[future]
                try:
                    objects[record["oid"]] = future.result()
                except Exception as error:
                    errors.append(f"{record['path']}: {error}")
                # Persist each successful result atomically: interruption or a
                # failed sibling must not force completed downloads to repeat.
                write_cache(path, data)
        if errors:
            raise RuntimeError("Gallery metadata extraction failed (successful entries retained):\n" + "\n".join(errors))
        return data


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workers", type=int, default=6)
    parser.add_argument("--refresh", action="store_true")
    args = parser.parse_args()
    data = ensure_metadata(workers=args.workers, refresh=args.refresh)
    print(f"Metadata cached for {len(data['objects'])} objects: {cache_path()}")


if __name__ == "__main__":
    main()
