set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

check:
    cargo run -p exif-tool --release -- check static/gallery

build:
    trunk build
