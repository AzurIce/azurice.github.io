from copy import deepcopy
from html.parser import HTMLParser
from html import escape
import json
from pathlib import Path
import re
import struct
import sys
import unittest
import zlib
from unittest.mock import patch
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools"))
import gallery_metadata as metadata
import site_prepare as prepare
from site_summaries import extract_html_summary, VOID_TAGS

OUTPUT = ROOT / "target/notist-site"


def request():
    documents = {path.relative_to(ROOT / "docs").as_posix(): path.read_text()
                 for path in (ROOT / "docs").rglob("*.md")}
    return {"version": 2, "event": "before_compile", "root": str(ROOT), "documents": documents}


class HTML(HTMLParser):
    def __init__(self, source):
        super().__init__(convert_charrefs=True)
        self.links, self.code, self.current, self.images, self.comment_paths = [], [], None, [], []
        self.kinds = []
        self.summaries, self.summary_parts, self.summary_depth = [], [], 0
        self.feed(source)

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if self.summary_depth:
            self.summary_parts.append(self.get_starttag_text())
            if tag not in VOID_TAGS:
                self.summary_depth += 1
        elif attrs.get("class") == "summary":
            self.summary_depth = 1
            self.summary_parts = []
        if tag == "pre":
            self.current = []
        if tag in {"a", "link", "img", "script"}:
            value = attrs.get("href", attrs.get("src"))
            if value:
                self.links.append(value)
        if tag == "img":
            self.images.append(attrs)
        if attrs.get("id") == "waline":
            self.comment_paths.append(attrs.get("data-path"))
        if tag == "body":
            self.kinds.append(attrs.get("data-kind"))

    def handle_endtag(self, tag):
        if self.summary_depth:
            if tag not in VOID_TAGS:
                self.summary_depth -= 1
            if self.summary_depth:
                self.summary_parts.append(f"</{tag}>")
            else:
                self.summaries.append("".join(self.summary_parts))
        if tag == "pre" and self.current is not None:
            self.code.append("".join(self.current).strip())
            self.current = None

    def handle_data(self, value):
        if self.summary_depth:
            self.summary_parts.append(escape(value, quote=False))
        if self.current is not None:
            self.current.append(value)


def code_blocks(source):
    blocks, current, indent = [], None, 0
    for line in source.splitlines():
        line = re.sub(r"^> ?", "", line)
        marker = re.match(r"^(\s*)`{3,}", line)
        if marker:
            if current is None:
                current, indent = [], len(marker[1])
            else:
                blocks.append("\n".join(current).strip())
                current = None
        elif current is not None:
            leading = len(line) - len(line.lstrip(" "))
            current.append(line[min(indent, leading):])
    return blocks


class PrepareTests(unittest.TestCase):
    def setUp(self):
        self.metadata_cache = metadata.read_cache(metadata.cache_path(ROOT))
        loader = patch.object(prepare, "ensure_metadata", return_value=self.metadata_cache)
        loader.start()
        self.addCleanup(loader.stop)

    def test_original_markdown_body_and_identity_preserved(self):
        inputs = request()
        originals = deepcopy(inputs["documents"])
        prepared = prepare.prepare(inputs)["documents"]
        for key, source in originals.items():
            self.assertTrue(prepared[key].endswith(source), key)
        self.assertIn('comment_path: "/posts/launchd.html"', prepared["posts/launchd.md"])
        self.assertFalse(any("launchd-md" in key for key in prepared))
        self.assertIn('comment_path: "/"', prepared["index.md"])

    def test_typst_conversion_preserves_code(self):
        for path in (ROOT / "docs/posts").glob("*.typ"):
            title, source = prepare.convert_typst(path.read_text())
            self.assertTrue(source.lstrip().startswith("# " + title))
            self.assertEqual(code_blocks(source), code_blocks(path.read_text()))

    def test_typst_conversion_preserves_shell_language(self):
        original = '#set document(title: "Shell")\n#title()\n```shell-unix-generic\nroot@host ~# neofetch\n```'
        _, converted = prepare.convert_typst(original)
        self.assertIn('```shell-unix-generic\n', converted)

    def test_unsupported_typst_fails_instead_of_silently_rendering(self):
        for body in ['#image("example.png")', '正文 #strong[你好]', '$x^2$']:
            with self.subTest(body=body), self.assertRaises(ValueError):
                prepare.convert_typst('#set document(title: "Test")\n#title()\n' + body)

    def test_virtual_entry_collision_fails(self):
        inputs = request()
        inputs["documents"]["posts/index.md"] = "Existing list"
        with self.assertRaisesRegex(ValueError, "collides"):
            prepare.prepare(inputs)

    def test_typst_markdown_collision_fails(self):
        inputs = request()
        inputs["documents"]["posts/linux-kernel-module.md"] = "Existing article"
        with self.assertRaisesRegex(ValueError, "collides"):
            prepare.prepare(inputs)

    def test_metadata_is_required_and_content_addressed(self):
        inputs = request()
        cache = self.metadata_cache["objects"]
        cache.pop(next(iter(cache)))
        with self.assertRaisesRegex(ValueError, "missing/stale"):
            prepare.prepare(inputs)

    def test_gallery_counts_years_and_descriptions(self):
        result = prepare.prepare(request())
        categories = result["blog"]["categories"]
        self.assertEqual(sum(category["count"] for category in categories), 306)
        for category in categories:
            self.assertEqual(sum(year["count"] for year in category["years"]), category["count"])
            images = [image for date in category["timeline"] for group in date["groups"] for image in group["images"]]
            self.assertEqual(len(images), category["count"])
            self.assertTrue(all(image["width"] > 0 and image["height"] > 0 for image in images))
        self.assertIn("gallery/ffxiv/2026-07-09 想导的猫/index.md", result["documents"])
        self.assertIn("在地球 OL", result["documents"]["gallery/地球 OL/index.md"])

    def test_search_text_strips_formatting_and_quoted_code(self):
        source = '# Title\n\n**Strong** and `code` [link](https://example.com)\n\n> ```cpp\n> SECRET_CODE\n> ```'
        self.assertEqual(prepare.plain_text(source), "Strong and code link")

    def test_summaries_use_rendered_html_not_search_text(self):
        extension = prepare.BlogExtension()
        compiled = extension(request())
        self.assertEqual(set(compiled), {"documents"})
        pages = [{"route": {"path": post["source_path"]}, "html": '<h1>Title</h1><p><em>Rendered</em> body</p><pre><code>example()</code></pre>'}
                 for post in extension.blog["posts"]]
        response = extension({"event": "before_template", "pages": pages, "data": {"other": 42}})
        self.assertEqual(set(response), {"data"})
        self.assertEqual(response["data"]["other"], 42)
        result = response["data"]["blog"]["posts"]
        for post in result:
            self.assertIn("<em>Rendered</em>", post["summary_html"])
            self.assertIn("<pre><code>example()</code></pre>", post["summary_html"])
            self.assertNotIn("<h1>", post["summary_html"])
            self.assertNotIn("summary", post)
        self.assertIn("launchctl", next(post["search"] for post in result if post["url"].endswith("launchd.html")))

    def test_missing_final_page_is_not_silently_replaced(self):
        extension = prepare.BlogExtension()
        extension(request())
        with self.assertRaises(KeyError):
            extension({"event": "before_template", "pages": [], "data": {}})


class SummaryTests(unittest.TestCase):
    def test_preserves_blocks_emphasis_code_and_secondary_headings(self):
        source = '<h1>Title <em>not excerpted</em></h1><h2>环境</h2><p><strong>Example</strong></p><pre><code><span class="z-keyword">int</span> x;</code></pre>'
        self.assertEqual(extract_html_summary(source, "/posts/a.html"), source[source.index("<h2>"):])

    def test_budget_counts_unicode_but_not_whitespace(self):
        text = "中" * 199 + " \n\t" + "文" + "截断部分"
        self.assertEqual(extract_html_summary("<p>" + text + "</p>", "/posts/a.html"), "<p>" + "中" * 199 + " \n\t文...</p>")

    def test_truncation_closes_nested_tags(self):
        source = '<section><h2><em>abcde</em></h2></section>'
        self.assertEqual(extract_html_summary(source, "/posts/a.html", 3), '<section><h2><em>abc...</em></h2></section>')

    def test_exact_length_and_short_excerpts_have_no_ellipsis(self):
        self.assertEqual(extract_html_summary("<p>abcd</p>", "/posts/a.html", 4), "<p>abcd</p>")
        self.assertEqual(extract_html_summary("<p>ab</p>", "/posts/a.html", 4), "<p>ab</p>")

    def test_entities_count_as_visible_characters_without_being_split(self):
        self.assertEqual(extract_html_summary("<p>&lt;x</p>", "/posts/a.html", 1), "<p>&lt;...</p>")
        self.assertEqual(extract_html_summary("<p>&lt;x</p>", "/posts/a.html", 2), "<p>&lt;x</p>")
        self.assertEqual(extract_html_summary("<p>&#39;&#x1f600;x</p>", "/posts/a.html", 2), "<p>'😀...</p>")

    def test_void_tags_and_rebased_links_do_not_create_invalid_markup(self):
        source = '<p id="duplicate"><img src="../images/a.png"><a href="#section">abcdef</a><br></p>'
        expected = '<p><img src="/images/a.png"><a href="/posts/a.html#section">abc...</a></p>'
        self.assertEqual(extract_html_summary(source, "/posts/a.html", 3), expected)

    def test_external_links_and_code_escapes_are_preserved(self):
        source = '<p><a href="https://example.com/?a=1&amp;b=2">link</a><code>&lt;T&gt;</code></p>'
        self.assertEqual(extract_html_summary(source, "/posts/a.html"), source)


class MetadataTests(unittest.TestCase):
    def test_xmp_attributes_and_elements(self):
        for body in ['xmp:Rating="5" xmp:Label="Red"', '']:
            content = '' if body else '<xmp:Rating>4</xmp:Rating><xmp:Label>Blue</xmp:Label>'
            xml = f'<x:xmpmeta xmlns:x="adobe:ns:meta/"><r xmlns:xmp="http://ns.adobe.com/xap/1.0/" {body}>{content}</r></x:xmpmeta>'
            result = metadata.xmp_fields(xml.encode())
            self.assertEqual(result["rating"], 5 if body else 4)
            self.assertEqual(result["label"], "Red" if body else "Blue")

    def test_exif_both_date_separator_formats(self):
        from PIL import Image
        for date in ["2024:09:15 16:54:11", "2024-09-15 16:54:11"]:
            exif = Image.Exif()
            exif[34665] = {36867: date}
            self.assertEqual(metadata.exif_fields(exif.tobytes())["created"], "2024-09-15T16:54:11")

    def test_png_header_dimensions(self):
        payload = struct.pack(">IIBBBBB", 1920, 1080, 8, 2, 0, 0, 0)
        def chunk(kind, body):
            return struct.pack(">I", len(body)) + kind + body + struct.pack(">I", zlib.crc32(kind + body) & 0xffffffff)
        fields = metadata.png_fields(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", payload) + chunk(b"IEND", b""))
        self.assertEqual((fields["width"], fields["height"]), (1920, 1080))

    def test_complete_object_must_match_shadow_hash(self):
        import io
        record = {"size": 4, "oid": "sha256:" + "0" * 64}
        with self.assertRaisesRegex(ValueError, "SHA-256"):
            metadata.verified_copy(io.BytesIO(b"data"), io.BytesIO(), record)


class ArtifactTests(unittest.TestCase):
    def test_expected_pages_and_real_404(self):
        self.assertTrue((OUTPUT / "_notist/publication.json").exists(), "run just build first")
        self.assertEqual(len(list(OUTPUT.rglob("*.html"))), 16)  # 15 Notist pages + the preserved XIV app
        self.assertEqual(HTML((OUTPUT / "404.html").read_text()).kinds, ["not_found"])
        self.assertFalse((OUTPUT / "posts/launchd-md").exists())

    def test_all_local_links_resolve(self):
        for path in OUTPUT.rglob("*.html"):
            if "xiv-market-lite" in path.parts:
                continue  # External application content is preserved, not rewritten.
            for link in HTML(path.read_text()).links:
                url = urlsplit(link)
                if url.scheme or url.netloc or not url.path:
                    continue
                destination = OUTPUT / unquote(url.path).lstrip("/") if url.path.startswith("/") else path.parent / unquote(url.path)
                self.assertTrue(destination.exists(), f"{path.relative_to(OUTPUT)} -> {link}")

    def test_every_post_code_block_and_comment_path(self):
        for path in (ROOT / "docs/posts").iterdir():
            if path.suffix not in {".md", ".typ"}:
                continue
            rendered = HTML((OUTPUT / "posts" / path.with_suffix(".html").name).read_text())
            self.assertEqual(rendered.code, code_blocks(path.read_text()), path.name)
            self.assertEqual(rendered.comment_paths, [f"/posts/{path.stem}.html"])
        self.assertEqual(HTML((OUTPUT / "index.html").read_text()).comment_paths, ["/"])

    def test_assets_and_gallery_coverage(self):
        self.assertEqual((OUTPUT / "avatar.jpg").read_bytes(), (ROOT / "site/static/avatar.jpg").read_bytes())
        self.assertFalse((OUTPUT / "static/avatar.jpg").exists())
        self.assertFalse((ROOT / "target/site-static/static").exists())
        self.assertEqual((OUTPUT / "xiv-market-lite/index.html").read_bytes(), (ROOT / "xiv-market-lite/index.html").read_bytes())
        for name in ["main.css", "site.js", "github.svg", "_notist/highlight.css"]:
            self.assertTrue((OUTPUT / name).is_file())
        count = 0
        for category in ["VRChat", "ffxiv", "地球 OL", "异环", "终末地"]:
            page = HTML((OUTPUT / "gallery" / category / "index.html").read_text())
            photos = [image for image in page.images if image.get("class") == "gallery-thumb"]
            count += len(photos)
            self.assertTrue(all(int(image["width"]) > 0 and int(image["height"]) > 0 for image in photos))
        self.assertEqual(count, 306)

    def test_source_resources_are_explicitly_still_downloadable(self):
        for path in (ROOT / "docs/posts").glob("*.typ"):
            self.assertEqual((OUTPUT / "posts" / path.name).read_bytes(), path.read_bytes())

    def test_cards_have_rich_truncated_excerpts(self):
        listing = HTML((OUTPUT / "posts/index.html").read_text())
        self.assertEqual(len(listing.summaries), 4)
        for summary in listing.summaries:
            self.assertIn("<pre>", summary)
            self.assertIn("<code", summary)
            self.assertIn("...", summary)
            self.assertNotIn("<h1", summary)
            self.assertNotIn(' id="', summary)
        self.assertIn("Ubuntu clang version", listing.summaries[0])
        search = HTML((OUTPUT / "search/index.html").read_text())
        self.assertEqual(search.summaries, listing.summaries)


if __name__ == "__main__":
    unittest.main()
