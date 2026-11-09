"""Retain source documents, generate pages, then enrich final template data."""

from collections import defaultdict
from datetime import datetime
import json
from pathlib import Path
import re
import subprocess
from urllib.parse import quote

from gallery_metadata import CACHE_FORMAT_VERSION, EXTRACTOR_VERSION, ensure_metadata, references, valid_entry
from site_summaries import extract_html_summary
from site_extension import serve


def annotation(**attrs):
    return "@!(" + ", ".join(f"{key}: {json.dumps(value, ensure_ascii=False)}" for key, value in attrs.items()) + ")\n"


def convert_typst(source):
    """Only the currently used title/heading/prose/raw-code subset, not Typst evaluation."""
    header = re.match(r'#set document\(title: ("[^"\n]+")\)\n', source)
    if not header:
        raise ValueError("unsupported Typst header; add an explicit converter rather than silently losing content")
    title = json.loads(header[1])
    result, fence = [], None
    for line in source[header.end():].splitlines():
        marker = re.match(r"^(\s*)(`{3,})(.*)$", line)
        if marker:
            if fence is None:
                fence = len(marker[2])
            elif len(marker[2]) >= fence and not marker[3].strip():
                fence = None
            result.append(line)
        elif fence is not None:
            result.append(line)
        elif line == "#title()":
            result.append("# " + title)
        elif re.match(r"^=+ ", line):
            heading, text = line.split(" ", 1)
            result.append("#" * (len(heading) + 1) + " " + text)
        elif re.search(r"(^|\s)#[A-Za-z_]", line) or re.search(r"(?<!`)\$[^`]*\$", line):
            raise ValueError(f"unsupported Typst expression outside raw code: {line}")
        else:
            result.append(line)
    if fence is not None:
        raise ValueError("unclosed Typst raw block")
    return title, "\n".join(result) + "\n"


def plain_text(source):
    lines, fence = [], None
    for line in source.splitlines():
        line = re.sub(r"^\s*> ?", "", line)
        marker = re.match(r"^\s*(`{3,}|~{3,})", line)
        if marker:
            if fence is None:
                fence = marker[1][0]
            elif marker[1][0] == fence:
                fence = None
        elif fence is None and not re.match(r"^\s*#+ ", line):
            lines.append(line)
    text = " ".join(lines)
    text = re.sub(r"!?\[([^\]]*)\]\([^)]*\)", r"\1", text)
    text = re.sub(r"<[^>]*>", "", text)
    text = re.sub(r"`+([^`]*)`+", r"\1", text)
    text = re.sub(r"(?<!\w)[*_~]+|[*_~]+(?!\w)", "", text)
    return re.sub(r"\s+", " ", text).strip()


def git_dates(root, path):
    timestamps = subprocess.run(["git", "log", "--follow", "--format=%cI", "--", path],
                                 cwd=root, check=True, capture_output=True, text=True).stdout.splitlines()
    if not timestamps:
        return "", "", 0
    def display(value):
        date = datetime.fromisoformat(value)
        return f"{date.year}-{date.month}-{date.day}"
    return display(timestamps[-1]), display(timestamps[0]), int(datetime.fromisoformat(timestamps[-1]).timestamp())


def filename_date(name):
    for pattern in (r"(\d{4})(\d{2})(\d{2})_(\d{2})(\d{2})(\d{2})",
                    r"(\d{4})-(\d{2})-(\d{2})[ _](\d{2})[-:]?(\d{2})[-:]?(\d{2})"):
        match = re.search(pattern, name)
        if match:
            try:
                return datetime(*map(int, match.groups())).isoformat()
            except ValueError:
                pass
    return ""


def gallery_data(root, docs, metadata, records=None):
    if (metadata.get("version") != CACHE_FORMAT_VERSION or metadata.get("extractor_version") != EXTRACTOR_VERSION
            or not isinstance(metadata.get("objects"), dict)):
        raise ValueError("gallery metadata cache missing/stale")
    grouped = defaultdict(lambda: defaultdict(list))
    for record in references(root) if records is None else records:
        relative = record["path"]
        meta = metadata["objects"].get(record["oid"])
        if not valid_entry(meta, record):
            raise ValueError(f"gallery metadata missing/stale for {relative}; run python3 tools/gallery_metadata.py")
        parts = Path(relative).parts
        # A folder is one recursive album, matching aoike's immediate-child grouping.
        group = parts[1] if len(parts) > 2 else ""
        created = meta.get("created") or filename_date(parts[-1])
        description = meta.get("description", "")
        if re.fullmatch(r"(?:0x)?0+", description, re.IGNORECASE):
            description = ""  # Camera placeholder, not a human photo caption.
        image = {"src": record["url"], "path": relative, "width": meta["width"], "height": meta["height"],
                 "title": meta.get("title") or Path(parts[-1]).stem.replace("_", " ").replace("-", " "),
                 "description": description, "created": created,
                 "rating": max(0, min(int(meta.get("rating", 0)), 5)), "label": meta.get("label", "")}
        grouped[parts[0]][group].append(image)
    categories = []
    for name, groups in sorted(grouped.items()):
        category_path = f"gallery/{name}/index.md"
        url = f"/gallery/{quote(name, safe='')}/"
        description = root / "gallery" / name / "index.md"
        docs[category_path] = annotation(title="Gallery", kind="gallery", category=name, comment_path=url) + (description.read_text() if description.exists() else "")
        timeline = defaultdict(list)
        year_counts = defaultdict(int)
        total = 0
        for group_name, images in sorted(groups.items()):
            total += len(images)
            if group_name:
                newest = max((image["created"] for image in images), default="")
                date = newest[:10]
                desc_path = root / "gallery" / name / group_name / "index.md"
                description_key = ""
                if desc_path.exists():
                    description_key = f"gallery/{name}/{group_name}/index.md"
                    docs[description_key] = annotation(title=group_name, kind="gallery", category=name, comment_path=url) + desc_path.read_text()
                timeline[date].append({"name": group_name, "images": images, "description_path": description_key})
                year_counts[date[:4] or "未知"] += len(images)
            else:
                for image in sorted(images, key=lambda image: image["created"], reverse=True):
                    date = image["created"][:10]
                    existing = next((group for group in timeline[date] if not group["name"]), None)
                    if existing is None:
                        existing = {"name": "", "images": [], "description_path": ""}
                        timeline[date].append(existing)
                    existing["images"].append(image)
                    year_counts[date[:4] or "未知"] += 1
        dates = []
        for date, albums in sorted(timeline.items(), reverse=True):
            display = "日期未知"
            if date:
                dt = datetime.fromisoformat(date)
                display = f"{dt.year}年{dt.month}月{dt.day}日"
            dates.append({"date": date, "display": display, "year": date[:4] or "未知", "groups": albums})
        categories.append({"name": name, "url": url, "count": total, "timeline": dates,
                           "years": [{"name": year, "count": count} for year, count in sorted(year_counts.items())]})
    return categories


def prepare(request):
    root = Path(request["root"])
    docs = request["documents"]
    posts = []
    for path in sorted((root / "docs/posts").rglob("*")):
        if path.suffix not in {".md", ".typ"} or not path.is_file():
            continue
        original_key = path.relative_to(root / "docs").as_posix()
        source = path.read_text() if path.suffix == ".typ" else docs[original_key]
        if path.suffix == ".typ":
            title, source = convert_typst(source)
        else:
            first = re.search(r"(?m)^#+ (.+)$", source)
            title = first[1] if first else path.stem
        key = Path(original_key).with_suffix(".md").as_posix()
        if path.suffix == ".typ" and key in docs:
            raise ValueError(f"Typst conversion collides with existing Markdown: {key}")
        public_path = "/" + quote(Path(key).with_suffix(".html").as_posix(), safe="/")
        created, updated, timestamp = git_dates(root, path.relative_to(root).as_posix())
        docs[key] = annotation(title=title, kind="article", created=created, updated=updated, comment_path=public_path) + source
        plain = plain_text(source)
        posts.append({"title": title, "url": public_path, "created": created, "updated": updated,
                      "timestamp": timestamp, "source_path": key, "search": title + " " + plain})
    posts.sort(key=lambda post: post["timestamp"], reverse=True)
    docs["index.md"] = annotation(title="冰弦のBlog", kind="home", comment_path="/") + docs["index.md"]
    for key, title, kind in [("posts/index.md", "所有文章", "posts"), ("gallery/index.md", "Gallery", "gallery_index"),
                              ("search/index.md", "搜索", "search"), ("404.md", "页面未找到", "not_found")]:
        if key in docs:
            raise ValueError(f"virtual entry collides with an existing document: {key}")
        docs[key] = annotation(title=title, kind=kind)
    records = references(root)
    metadata = ensure_metadata(root, records=records)
    categories = gallery_data(root, docs, metadata, records)
    return {"documents": docs, "blog": {"posts": posts, "categories": categories}}


class BlogExtension:
    def __init__(self):
        self.blog = None

    def __call__(self, request):
        if request["event"] == "before_compile":
            result = prepare(request)
            self.blog = result["blog"]
            return {"documents": result["documents"]}
        if self.blog is None:
            raise ValueError("before_template requires before_compile")
        pages = {page["route"]["path"]: page for page in request["pages"]}
        for post in self.blog["posts"]:
            page = pages[post["source_path"]]
            post["summary_html"] = extract_html_summary(page["html"], post["url"])
        return {"data": {**request["data"], "blog": self.blog}}


if __name__ == "__main__":
    serve(BlogExtension())
