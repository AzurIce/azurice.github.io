"""Compare the paired top-60px screenshots emitted by nav-parity.test.mjs."""

import json
from pathlib import Path

from PIL import Image, ImageChops

root = Path(__file__).resolve().parents[2] / "target/notist-verification/navigation"
results = []
for path in sorted(root.glob("[0-9]*-deployed.png")):
    other = path.with_name(path.name.replace("-deployed", "-notist"))
    with Image.open(path) as image:
        expected = image.convert("RGB")
    with Image.open(other) as image:
        actual = image.convert("RGB")
    if expected.size != actual.size:
        raise SystemExit(f"Screenshot size mismatch: {path.name}")
    diff = ImageChops.difference(expected, actual)
    count = sum(max(pixel) > 0 for pixel in diff.getdata())
    results.append({"case": path.stem.removesuffix("-deployed"), "different_pixels": count,
                    "total_pixels": expected.width * expected.height,
                    "max_channel_difference": max(high for low, high in diff.getextrema())})
(root / "pixel-diff.json").write_text(json.dumps(results, indent=2) + "\n")
if len(results) != 18 or any(result["different_pixels"] for result in results):
    raise SystemExit("Navigation pixels differ; see target/notist-verification/navigation/pixel-diff.json")
print("PASS navigation pixels: all 18 paired top-60px screenshots are identical")
