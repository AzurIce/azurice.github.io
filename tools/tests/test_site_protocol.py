"""Interactive exchanges must respond before EOF, including no-op events."""
import json
from pathlib import Path
import selectors
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]


class ProtocolTests(unittest.TestCase):
    def test_blog_keeps_build_state_until_final_pages_arrive(self):
        script = (
            'import site_prepare as p; from site_extension import serve; '
            'p.prepare = lambda request: {"documents": request["documents"], '
            '"blog": {"posts": [{"source_path": "post.md", "url": "/post.html"}], "categories": []}}; '
            'serve(p.BlogExtension())'
        )
        process = subprocess.Popen([sys.executable, "-c", script], cwd=ROOT / "tools",
                                   stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        selector = selectors.DefaultSelector()
        selector.register(process.stdout, selectors.EVENT_READ)
        try:
            requests = [
                {"version": 2, "event": "before_compile", "documents": {"post.md": "# Post"}},
                {"version": 2, "event": "before_template", "data": {"keep": 42},
                 "pages": [{"route": {"path": "post.md"}, "html": "<h1>Post</h1><p><em>Final HTML</em></p>"}]},
            ]
            for request in requests:
                process.stdin.write(json.dumps(request) + "\n")
                process.stdin.flush()
                self.assertTrue(selector.select(5), "extension waited for EOF or failed to flush")
                response = json.loads(process.stdout.readline())
                self.assertEqual(response["version"], 2)
                self.assertEqual(response["event"], request["event"])
                if request["event"] == "before_compile":
                    self.assertEqual(set(response), {"version", "event", "documents"})
                    self.assertEqual(response["documents"], request["documents"])
                else:
                    self.assertEqual(set(response), {"version", "event", "data"})
                    self.assertEqual(response["data"]["keep"], 42)
                    self.assertEqual(response["data"]["blog"]["posts"][0]["summary_html"], "<p><em>Final HTML</em></p>")
            process.stdin.close()
            self.assertEqual(process.wait(timeout=5), 0)
            self.assertEqual(process.stdout.read(), "")
        finally:
            if process.poll() is None:
                process.kill()
            process.wait()
            selector.close()
            process.stdout.close()
            process.stderr.close()

    def test_assets_flush_two_events_and_reject_old_protocol(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "site/styles/uno.css"
            source.parent.mkdir(parents=True)
            source.write_text("body {}")
            process = subprocess.Popen([sys.executable, str(ROOT / "tools/site_assets.py")],
                                       stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
            try:
                selector = selectors.DefaultSelector()
                selector.register(process.stdout, selectors.EVENT_READ)
                for request in [
                    {"version": 2, "event": "before_compile", "root": directory, "documents": {}},
                    {"version": 2, "event": "before_template", "root": directory, "pages": [], "data": {"keep": 1}},
                    {"version": 1, "event": "before_compile", "root": directory, "documents": {}},
                ]:
                    process.stdin.write(json.dumps(request) + "\n")
                    process.stdin.flush()
                    self.assertTrue(selector.select(5), "extension waited for EOF or did not flush")
                    response = json.loads(process.stdout.readline())
                    self.assertEqual(response["event"], request["event"])
                    self.assertEqual(response["version"], 2)
                    self.assertEqual(bool(response.get("errors")), request["version"] != 2)
                    self.assertFalse({"documents", "pages", "data", "input"} & response.keys())
                process.stdin.close()
                self.assertEqual(process.wait(timeout=5), 0)
                self.assertEqual(process.stdout.read(), "")
                selector.close()
            finally:
                if process.poll() is None:
                    process.kill()
                process.wait()
                process.stdout.close()
                process.stderr.close()
