import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools"))
import site_assets


class SiteAssetTests(unittest.TestCase):
    def test_asset_extension_protocol_and_refresh(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "site/styles/uno.css"
            source.parent.mkdir(parents=True)
            source.write_text(".nav-btn { display: flex; }")
            request = {"version": 2, "event": "before_compile", "root": directory, "documents": {"index.md": "Home"}}
            response = subprocess.run([sys.executable, str(ROOT / "tools/site_assets.py")],
                                      input=json.dumps(request) + "\n", capture_output=True, text=True, check=True)
            self.assertEqual(json.loads(response.stdout), {"version": 2, "event": "before_compile"})
            target = root / "target/site-static/css/uno.css"
            self.assertEqual(target.read_bytes(), source.read_bytes())
            stamp = target.stat().st_mtime_ns
            site_assets.prepare(request)
            self.assertEqual(target.stat().st_mtime_ns, stamp)
            source.write_text(".nav-btn { display: grid; }")
            site_assets.prepare(request)
            self.assertEqual(target.read_bytes(), source.read_bytes())
            source.unlink()
            with self.assertRaises(FileNotFoundError):
                site_assets.prepare(request)

    def test_unsupported_protocol_fails(self):
        with self.assertRaisesRegex(ValueError, "protocol"):
            site_assets.prepare({"version": 1, "stage": "prepare"})

    @unittest.skipUnless((ROOT / "target/notist-site/index.html").exists(), "build first")
    def test_published_legacy_css_and_load_order(self):
        output = ROOT / "target/notist-site"
        self.assertEqual((output / "css/uno.css").read_bytes(),
                         (ROOT / "site/styles/uno.css").read_bytes())
        html = (output / "index.html").read_text()
        self.assertLess(html.index('href="/css/uno.css"'), html.index('href="/main.css"'))
        css = (output / "main.css").read_text()
        # Rules absent from the former hand-port verify original SCSS is compiled.
        self.assertIn(".markdown blockquote cite", css)
        self.assertIn(".markdown figure figcaption", css)
