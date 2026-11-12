#!/usr/bin/env python3
"""Build the single-page white paper website from the markdown source.

Usage: python3 scripts/build_site.py docs/existenz_wp.md index.html

The script is dependency-free (standard library only) so it runs both locally
and in GitHub Actions. It produces one self-contained index.html: fixed
left-side table of contents, sticky title banner with breadcrumb and tagline,
orange visual identity, airy typography with bold lead-in labels, inline CSS,
no external assets.
"""
import html
import re
import sys

REPO_URL = "https://github.com/davidbenoitbetmalle/existenz"
DISCUSSION_URL = "https://github.com/davidbenoitbetmalle/existenz/discussions/2"


def parse_markdown(text):
    """Parse the subset of markdown used by the source: # / ## / ### headings,
    '- ' bullets, pipe tables, paragraphs (consecutive lines joined)."""
    blocks = []  # (kind, payload)
    lines = text.split("\n")
    i = 0
    table_buf = []
    para_buf = []

    def flush_para():
        if para_buf:
            blocks.append(("p", " ".join(para_buf)))
            para_buf.clear()

    def flush_table():
        if table_buf:
            blocks.append(("table", list(table_buf)))
            table_buf.clear()

    while i < len(lines):
        line = lines[i].rstrip()
        if line.strip().startswith("|"):
            flush_para()
            cells = [c.strip() for c in line.strip().strip("|").split("|")]
            if all(re.fullmatch(r":?-{2,}:?", c) for c in cells):
                i += 1
                continue
            table_buf.append(cells)
            i += 1
            continue
        flush_table()
        s = line.strip()
        if not s:
            flush_para()
            i += 1
            continue
        if s.startswith("### "):
            flush_para()
            blocks.append(("h3", s[4:]))
        elif s.startswith("## "):
            flush_para()
            blocks.append(("h2", s[3:]))
        elif s.startswith("# "):
            flush_para()
            blocks.append(("h1", s[2:]))
        elif s.startswith("- "):
            flush_para()
            blocks.append(("li", s[2:]))
        else:
            para_buf.append(s)
        i += 1
    flush_para()
    flush_table()
    return blocks


def inline(text):
    """Render inline markdown: **bold**, *italic*, and bold lead-in labels."""
    def bold_label(m):
        label, rest = m.group(1), m.group(2)
        return f"<strong>{html.escape(label)}:</strong>{html.escape(rest)}"
    # escape everything first
    out = html.escape(text, quote=False)
    # bold/italic emphasis written in the source
    out = re.sub(r"\*\*(.+?)\*\*", r"<strong>\1</strong>", out)
    out = re.sub(r"(?<![\w*])\*([^*\n]+?)\*(?![\w*])", r"<em>\1</em>", out)
    # bold lead-in labels: "Label: rest" with a short label, only at the start
    m = re.match(r"^([A-Z][^:]{2,48}):(\s+|$)", out)
    if m and '"' not in m.group(1):
        out = f"<strong>{m.group(1)}:</strong>" + out[m.end(1) + 1:]
    return out


CSS = """:root{--ink:#2a2018;--muted:#6b5d4a;--accent:#d95f18;--accent2:#a54a0e;--bg:#faf7f1;--line:#e9dfd0;--soft:#fdf3e7;--max:840px;--navw:280px;--heroh:170px}
*{box-sizing:border-box}
body{margin:0;font-family:Georgia,'Times New Roman',serif;background:var(--bg);color:var(--ink);line-height:1.8;font-size:17.5px}
.topbar{position:sticky;top:0;z-index:30;margin-left:var(--navw)}
header.hero{background:linear-gradient(135deg,#3a230d 0%,#7a3b08 60%,#a54a0e 100%);color:#fff;padding:28px 24px 22px;text-align:center}
header.hero h1{font-size:2rem;margin:0 0 8px;letter-spacing:.5px}
header.hero p.subtitle{margin:0 auto;max-width:640px;color:#f4d9b8;font-size:1.02rem;font-style:italic}
header.hero p.tagline{margin:12px auto 0;color:#ffb066;font-size:.9rem;font-style:italic}
.crumbs{background:#241505;color:#c99e6d;padding:8px 24px;font-family:system-ui,sans-serif;font-size:.82rem;text-align:center}
.crumbs a{color:#ffb066;text-decoration:none}
.crumbs a:hover{color:#fff;text-decoration:underline}
.crumbs .sep{margin:0 8px;color:#8a6a45}
nav{background:#fffdf8;border-right:1px solid var(--line);position:fixed;top:0;left:0;bottom:0;width:var(--navw);overflow:auto;padding:24px 18px;z-index:10}
nav summary{cursor:pointer;font-weight:bold;font-family:system-ui,sans-serif;font-size:.9rem;color:var(--accent2);list-style:none}
nav ul{padding:8px 0 4px;margin:6px 0 0;list-style:none}
nav li{margin:3px 0}
nav li.lvl2{padding-left:14px;font-size:.85rem}
nav a{color:var(--muted);text-decoration:none}
nav a:hover{color:var(--accent2);text-decoration:underline}
main{max-width:var(--max);padding:48px 48px 96px;margin-left:calc(var(--navw) + (100% - var(--navw) - var(--max))/2)}
h2{font-family:system-ui,sans-serif;font-size:1.65rem;font-weight:700;color:var(--accent2);border-bottom:3px solid var(--accent);padding-bottom:8px;margin:72px 0 20px;scroll-margin-top:calc(var(--heroh) + 28px)}
h3{font-family:system-ui,sans-serif;font-size:1.2rem;color:#7a3b08;margin-top:44px;scroll-margin-top:calc(var(--heroh) + 28px)}
h1{font-family:system-ui,sans-serif;scroll-margin-top:calc(var(--heroh) + 28px)}
p{margin:0 0 1.4em}
strong{font-weight:700;color:#1f1205}
ul{padding-left:26px;margin:0 0 1.6em}
li{margin:0 0 .9em;line-height:1.75}
table{border-collapse:collapse;width:100%;margin:24px 0 32px;font-size:.92rem;font-family:system-ui,sans-serif}
th,td{border:1px solid var(--line);padding:10px 12px;text-align:left;vertical-align:top}
th{background:var(--soft);color:#7a3b08}
footer{border-top:3px solid var(--accent);color:var(--muted);text-align:center;padding:28px;font-size:.85rem;font-family:system-ui,sans-serif;margin-left:var(--navw);background:var(--soft)}
@media(max-width:1100px){main{margin-left:var(--navw);padding:48px 24px 96px}}
@media(max-width:820px){
nav{position:static;width:auto;max-height:none;border-right:none;border-bottom:1px solid var(--line);padding:10px 20px}
.topbar,header.hero,main,footer{margin-left:0}
header.hero h1{font-size:1.5rem}
main{padding:40px 24px 80px}
}"""


def strip_md(text):
    """Remove markdown emphasis for plain rendering in the hero."""
    return re.sub(r"\*+", "", text).strip()


def build(blocks):
    # The first h1 is the paper title, the first h2 the subtitle, and the
    # first paragraph (if it appears before any other heading) the tagline.
    title = next(c for k, c in blocks if k == "h1")
    subtitle = next(c for k, c in blocks if k == "h2")
    tagline = ""
    for k, c in blocks:
        if k == "p":
            tagline = strip_md(c)
            break
        if k in ("h1", "h2"):
            continue
        if k == "h3":
            break

    body = []
    toc = []
    sid = 0
    seen_hero = {"h1": False, "h2": False, "p": False}
    for kind, content in blocks:
        if kind == "h1":
            if not seen_hero["h1"]:
                seen_hero["h1"] = True
                continue  # title goes to the hero
            sid += 1
            body.append(f'<h1 id="s{sid}">{html.escape(content)}</h1>')
            toc.append((1, content, f"s{sid}"))
        elif kind == "h2":
            if not seen_hero["h2"]:
                seen_hero["h2"] = True
                continue  # subtitle goes to the hero
            sid += 1
            body.append(f'<h2 id="s{sid}">{html.escape(content)}</h2>')
            toc.append((2, content, f"s{sid}"))
        elif kind == "p":
            if not seen_hero["p"] and not tagline == "":
                if strip_md(content) == tagline:
                    seen_hero["p"] = True
                    continue  # tagline goes to the hero
            body.append(f"<p>{inline(content)}</p>")
        elif kind == "h3":
            body.append(f"<h3>{inline(content)}</h3>")
        elif kind == "li":
            body.append(f"<li>{inline(content)}</li>")
        elif kind == "table":
            rows = content
            t = "<table><thead><tr>" + "".join(
                f"<th>{html.escape(c)}</th>" for c in rows[0]) + "</tr></thead><tbody>"
            for row in rows[1:]:
                t += "<tr>" + "".join(f"<td>{html.escape(c)}</td>" for c in row) + "</tr>"
            body.append(t + "</tbody></table>")

    main_html = re.sub(r"(<li>.*?</li>(?:\s*<li>.*?</li>)*)", r"<ul>\1</ul>",
                       "".join(body), flags=re.S)
    toc_html = "".join(
        f'<li class="lvl{n}"><a href="#{a}">{html.escape(t)}</a></li>'
        for n, t, a in toc)
    tagline_html = f'<p class="tagline">{html.escape(tagline)}</p>' if tagline else ""

    return f"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{html.escape(title)}</title>
<meta name="description" content="Existenz (EXZ): a fair global currency. The complete white paper: protocol, governance, honest limits.">
<style>
{CSS}
</style>
</head>
<body>
<nav><details open><summary>Table of contents</summary><ul>{toc_html}</ul></details></nav>
<div class="topbar">
<header class="hero"><h1>{html.escape(title)}</h1><p class="subtitle">{html.escape(strip_md(subtitle))}</p>{tagline_html}</header>
<div class="crumbs"><a href="{REPO_URL}">GitHub repository</a><span class="sep">/</span><a href="{DISCUSSION_URL}">Discussion: where this paper is most likely to be wrong</a></div>
</div>
<main>{main_html}</main>
<footer>Existenz White Paper, v3. A protocol-level proposal, published with its known limits stated openly.</footer>
</body></html>"""


def main():
    src, out = sys.argv[1], sys.argv[2]
    with open(src, encoding="utf-8") as f:
        text = f.read()
    # the hero title is the first h1 and the hero subtitle the first h2
    page = build(parse_markdown(text))
    with open(out, "w", encoding="utf-8") as f:
        f.write(page)
    print(f"Built {out} ({len(page)} bytes)")


if __name__ == "__main__":
    main()
