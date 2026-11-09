"""HTML excerpts from final Notist Pages, never from source regexes."""

from html import escape, unescape
from html.parser import HTMLParser
from urllib.parse import urljoin

VOID_TAGS = {"area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"}


class Excerpt(HTMLParser):
    def __init__(self, article_url, limit):
        super().__init__(convert_charrefs=False)
        self.article_url = article_url
        self.limit = limit
        self.count = 0
        self.skipping = 0
        self.truncated = False
        self.parts = []
        self.stack = []

    def finish_tags(self):
        while self.stack:
            self.parts.append(f"</{self.stack.pop()}>")

    def truncate(self):
        self.parts.append("...")
        self.finish_tags()
        self.truncated = True

    def handle_starttag(self, tag, attrs):
        if self.truncated:
            return
        if self.skipping or tag == "h1":
            if tag not in VOID_TAGS:
                self.skipping += 1
            return
        attributes = []
        for name, value in attrs:
            # Excerpts share a document with other excerpts. Original heading
            # ids must not create duplicate ids in the list/search page.
            if name == "id":
                continue
            if value is not None and name in {"href", "src", "poster"}:
                value = urljoin(self.article_url, value)
            attributes.append(name if value is None else f'{name}="{escape(value, quote=True)}"')
        self.parts.append("<" + tag + (" " + " ".join(attributes) if attributes else "") + ">")
        if tag not in VOID_TAGS:
            self.stack.append(tag)

    def handle_startendtag(self, tag, attrs):
        self.handle_starttag(tag, attrs)
        if tag not in VOID_TAGS:
            self.handle_endtag(tag)

    def handle_endtag(self, tag):
        if self.truncated:
            return
        if self.skipping:
            if tag not in VOID_TAGS:
                self.skipping -= 1
            return
        if tag in self.stack:
            while self.stack:
                current = self.stack.pop()
                self.parts.append(f"</{current}>")
                if current == tag:
                    break

    def handle_data(self, data):
        if self.truncated or self.skipping:
            return
        for character in data:
            if self.count >= self.limit:
                self.truncate()
                return
            self.parts.append(escape(character, quote=False))
            if not character.isspace():
                self.count += 1

    def entity(self, source):
        if self.truncated or self.skipping:
            return
        # Count the rendered character, not the renderer-specific spelling of
        # &#39; / &gt; etc. This also prevents emitting a partial HTML entity.
        self.handle_data(unescape(source))

    def handle_entityref(self, name):
        self.entity(f"&{name};")

    def handle_charref(self, name):
        self.entity(f"&#{name};")


def extract_html_summary(html, article_url, limit=200):
    if limit < 0:
        raise ValueError("summary limit must be nonnegative")
    parser = Excerpt(article_url, limit)
    parser.feed(html)
    parser.close()
    parser.finish_tags()
    return "".join(parser.parts).strip()
