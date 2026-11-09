"""Read-only Site prepare trial. Not a general Typst converter or aoike replacement."""

import json
from pathlib import Path
import re
import subprocess
import sys
import tomllib
from urllib.parse import quote, urlsplit


def annotation(**attrs):
    return "@!(" + ", ".join(f"{key}: {json.dumps(value, ensure_ascii=False)}" for key, value in attrs.items()) + ")\n"


def convert_typst(source):
    """Only the two current posts' title, heading and fenced-code subset."""
    title_match = re.match(r'#set document\(title: ("[^"\n]+")\)\n', source)
    if not title_match:
        raise ValueError("unsupported Typst document header")
    title = json.loads(title_match[1])
    result = []
    fence = None
    for line in source[title_match.end():].splitlines():
        marker = re.match(r"^(\s*)(`{3,})(.*)$", line)
        if marker:
            if fence is None:
                fence = len(marker[2])
                line = line.replace("shell-unix-generic", "bash")
            elif len(marker[2]) >= fence and not marker[3].strip():
                fence = None
            result.append(line)
        elif fence is not None:
            result.append(line)
        elif line == "#title()":
            result.append(f"# {title}")
        elif re.match(r"^=+ ", line):
            heading, text = line.split(" ", 1)
            result.append("#" * (len(heading) + 1) + " " + text)
        elif line.startswith("#"):
            raise ValueError(f"unsupported Typst expression: {line}")
        else:
            result.append(line)
    if fence is not None:
        raise ValueError("unclosed fenced code")
    return title, "\n".join(result) + "\n"


def dates(root, path):
    log = subprocess.run(
        ["git", "log", "--follow", "--format=%cI", "--", str(path)],
        cwd=root, check=True, capture_output=True, text=True,
    ).stdout.splitlines()
    return (log[-1], log[0]) if log else ("未提交", "未提交")


def summary(source):
    # Deliberately plain text and approximate, not aoike's HTML summary algorithm.
    text = re.sub(r"(?ms)^\s*```.*?^\s*```\s*$", "", source)
    text = re.sub(r"(?m)^#+ .*\n?", "", text)
    return re.sub(r"\s+", " ", text).strip()[:200]


def gallery(root):
    config = tomllib.loads((root / "shadow.toml").read_text())
    backend = config["backend"]
    if backend["type"] != "volcengine_tos":
        raise ValueError("trial only supports volcengine_tos")
    endpoint = urlsplit(backend["endpoint"])
    prefix = "/".join(quote(part, safe="") for part in [*filter(None, backend.get("prefix", "").split("/")), config["name"]])
    base = f"{endpoint.scheme}://{backend['bucket']}.{endpoint.netloc}/{prefix}/objects/sha256"
    categories = {}
    for ref in sorted((root / ".shadow/refs/gallery").rglob("*.ref")):
        path = ref.relative_to(root / ".shadow/refs/gallery")
        record = tomllib.loads(ref.read_text())
        if not re.fullmatch(r"sha256:[0-9a-fA-F]{64}", record["oid"]):
            raise ValueError(f"invalid oid: {ref}")
        oid = record["oid"][7:]
        name = path.name.removesuffix(".ref")
        group = "/".join(path.parts[1:-1]) or "未分组"
        categories.setdefault(path.parts[0], {}).setdefault(group, []).append({
            "name": name, "url": f"{base}/{oid[:2]}/{oid[2:]}", "size": record["size"],
        })
    return [{"name": name, "count": sum(map(len, groups.values())), "groups": [
        {"name": group, "images": images} for group, images in groups.items()
    ]} for name, groups in categories.items()]


def prepare(request):
    root = Path(request["root"])
    data = request["input"]
    docs = data["documents"]
    posts = []
    for path in sorted((root / "docs/posts").iterdir()):
        if path.suffix not in {".md", ".typ"}:
            continue
        original_key = path.relative_to(root / "docs").as_posix()
        source = path.read_text() if path.suffix == ".typ" else docs[original_key]
        if path.suffix == ".typ":
            title, source = convert_typst(source)
        else:
            title = re.search(r"(?m)^#+ (.+)$", source)[1]
        created, updated = dates(root, path.relative_to(root))
        key = str(Path(original_key).with_suffix(".md"))
        if path.suffix == ".typ" and key in docs:
            raise ValueError(f"converted Typst conflicts with existing document: {key}")
        public_path = "/" + str(Path(key).with_suffix(".html"))
        docs[key] = annotation(title=title, created=created, updated=updated,
                               original_source=str(path.relative_to(root)), comment_path=public_path) + source
        posts.append({"title": title, "created": created, "updated": updated,
                      "url": public_path, "summary": summary(source)})
    posts.sort(key=lambda post: post["created"], reverse=True)
    docs["index.md"] = annotation(title="冰弦のBlog", comment_path="/") + docs["index.md"]
    docs["posts/index.md"] = annotation(title="所有文章", kind="posts")
    docs["gallery/index.md"] = annotation(title="相册接入试验", kind="gallery")
    data["data"]["trial"] = {"posts": posts, "gallery": gallery(root)}
    return {"version": 1, "input": data}


if __name__ == "__main__":
    try:
        request = json.load(sys.stdin)
        if request["version"] != 1 or request["stage"] != "prepare":
            raise ValueError("unsupported protocol")
        json.dump(prepare(request), sys.stdout, ensure_ascii=False)
    except Exception as error:
        print(f"prepare trial: {error}", file=sys.stderr)
        sys.exit(1)
