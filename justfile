set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

build:
    python3 tools/build_site.py build

serve:
    python3 tools/build_site.py serve

check:
    python3 -m unittest discover -s tools/tests -v
    node --test tools/tests/gallery-layout.test.mjs
    node --check site/static/site.js

gallery-sync:
    python3 tools/gallery_metadata.py

check-browser:
    node tools/tests/browser.test.mjs

check-nav-parity:
    node tools/tests/nav-parity.test.mjs
    python3 tools/tests/nav-pixels.py

check-summary-parity:
    node tools/tests/summary-parity.test.mjs

build-legacy:
    trunk build

exif-check:
    cargo run -p exif-tool --release -- check gallery
