"""Prepare the frozen legacy UnoCSS asset before Notist collects static files."""

from pathlib import Path
import tempfile
from site_extension import serve


def prepare(request):
    if request.get("version") != 2 or request.get("event") not in {"before_compile", "before_template"}:
        raise ValueError("unsupported Site extension protocol")
    if request["event"] == "before_template":
        return {}
    root = Path(request["root"])
    source = root / "site/styles/uno.css"
    destination = root / "target/site-static/css/uno.css"
    content = source.read_bytes()  # Missing source must fail, never reuse stale CSS.
    destination.parent.mkdir(parents=True, exist_ok=True)
    if not destination.exists() or destination.read_bytes() != content:
        with tempfile.NamedTemporaryFile(dir=destination.parent, delete=False) as file:
            temporary = Path(file.name)
            file.write(content)
        try:
            temporary.replace(destination)
        finally:
            temporary.unlink(missing_ok=True)
    return {}


if __name__ == "__main__":
    serve(prepare)
