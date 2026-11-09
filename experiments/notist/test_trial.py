"""Unit tests plus artifact checks; run after building both trial manifests."""

from html.parser import HTMLParser
from pathlib import Path
import re
import unittest

from prepare import convert_typst, gallery

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "target/notist-trial"


class ParsedHTML(HTMLParser):
    def __init__(self, source):
        super().__init__(convert_charrefs=True)
        self.pre = []
        self.current = None
        self.links = []
        self.feed(source)

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if tag == "pre":
            self.current = []
        if tag in {"a", "link", "img", "script"}:
            value = attrs.get("href", attrs.get("src"))
            if value:
                self.links.append(value)

    def handle_endtag(self, tag):
        if tag == "pre" and self.current is not None:
            self.pre.append("".join(self.current).strip())
            self.current = None

    def handle_data(self, text):
        if self.current is not None:
            self.current.append(text)


def blocks(source):
    result = []
    current = None
    indent = 0
    for line in source.splitlines():
        line = re.sub(r"^> ?", "", line)
        marker = re.match(r"^(\s*)`{3,}", line)
        if marker:
            if current is None:
                current = []
                indent = len(marker[1])
            else:
                result.append("\n".join(current).strip())
                current = None
        elif current is not None:
            leading = len(line) - len(line.lstrip(" "))
            current.append(line[min(indent, leading):])
    return result


class TrialTests(unittest.TestCase):
    def test_typst_fences_are_preserved(self):
        for path in (ROOT / "docs/posts").glob("*.typ"):
            with self.subTest(path=path.name):
                title, converted = convert_typst(path.read_text())
                self.assertTrue(converted.lstrip().startswith("# " + title))
                self.assertEqual(blocks(path.read_text()), blocks(converted))
                self.assertNotIn("#set document", converted)

    def test_unsupported_typst_expression_fails(self):
        with self.assertRaisesRegex(ValueError, "unsupported Typst expression"):
            convert_typst('#set document(title: "test")\n#title()\n#image("x.png")\n')

    def test_native_page_counts(self):
        self.assertEqual(len(list((OUTPUT / "native").rglob("*.html"))), 3)
        self.assertEqual(len(list((OUTPUT / "native-gallery").rglob("*.html"))), 3)
        for path in (ROOT / "docs/posts").glob("*.typ"):
            self.assertEqual((OUTPUT / "native/posts" / path.name).read_bytes(), path.read_bytes())

    def test_enriched_page_count(self):
        self.assertEqual(len(list((OUTPUT / "enriched").rglob("*.html"))), 7)

    def test_no_legacy_routes_and_home_comment_identity(self):
        site = OUTPUT / "enriched"
        self.assertFalse((site / "posts/launchd-md").exists())
        self.assertIn('data-path="/"', (site / "index.html").read_text())

    def test_all_rendered_code_blocks_and_comment_paths(self):
        for path in (ROOT / "docs/posts").iterdir():
            with self.subTest(path=path.name):
                public_path = f"/posts/{path.stem}.html"
                rendered = (OUTPUT / "enriched" / public_path.lstrip("/")).read_text()
                self.assertEqual(ParsedHTML(rendered).pre, blocks(path.read_text()))
                self.assertIn(f'data-path="{public_path}"', rendered)
                self.assertIn('class="z-', rendered)

    def test_static_assets_and_local_links(self):
        site = OUTPUT / "enriched"
        self.assertEqual((site / "avatar.jpg").read_bytes(), (ROOT / "site/static/avatar.jpg").read_bytes())
        self.assertTrue((site / "trial.css").is_file())
        self.assertTrue((site / "_notist/highlight.css").is_file())
        for page in site.rglob("*.html"):
            for link in ParsedHTML(page.read_text()).links:
                if link.startswith(("http:", "https:", "#", "mailto:")):
                    continue
                destination = site / link.lstrip("/") if link.startswith("/") else page.parent / link
                self.assertTrue(destination.exists(), f"{page}: {link}")

    def test_gallery_ref_coverage(self):
        categories = gallery(ROOT)
        count = sum(category["count"] for category in categories)
        self.assertEqual(count, len(list((ROOT / ".shadow/refs/gallery").rglob("*.ref"))))
        expected = {ref.relative_to(ROOT / ".shadow/refs/gallery").parts[0]
                    for ref in (ROOT / ".shadow/refs/gallery").rglob("*.ref")}
        self.assertEqual({category["name"] for category in categories}, expected)
        rendered = (OUTPUT / "enriched/gallery/index.html").read_text()
        self.assertEqual(rendered.count('class="photo"'), count)
        self.assertNotIn(".ref\"", rendered)


if __name__ == "__main__":
    unittest.main()
