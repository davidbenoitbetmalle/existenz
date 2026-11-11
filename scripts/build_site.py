#!/usr/bin/env python3
"""Build the single-page white paper website from the markdown source.

Usage: python3 scripts/build_site.py docs/existenz_wp.md index.html

The script is dependency-free (standard library only) so it runs both locally
and in GitHub Actions. It produces one self-contained index.html: sticky
table of contents, inline CSS, no external assets.
"""
import html
import re
import sys


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


CSS = """:root{--ink:#1a2333;--muted:#5a6578;--accent:#1f5fa8;--bg:#fdfdfb;--line:#e3e6ec;--max:860px}
*{box-sizing:border-box}
body{margin:0;font-family:Georgia,'Times New Roman',serif;background:var(--bg);color:var(--ink);line-height:1.65;font-size:17px}
header.hero{background:var(--ink);color:#fff;padding:64px 24px 56px;text-align:center}
header.hero h1{font-size:2.4rem;margin:0 0 8px;letter-spacing:.5px}
header.hero p{margin:0 auto;max-width:640px;color:#c6cfdd;font-size:1.15rem;font-style:italic}
nav{background:#fff;border-bottom:1px solid var(--line);position:sticky;top:0;z-index:10;max-height:44vh;overflow:auto}
nav .inner{max-width:var(--max);margin:0 auto;padding:10px 20px}
nav summary{cursor:pointer;font-weight:bold;font-family:system-ui,sans-serif;font-size:.9rem;color:var(--accent);list-style:none}
nav ul{columns:2;column-gap:40px;padding:8px 0 4px;margin:6px 0 0;list-style:none}
nav li{margin:2px 0;break-inside:avoid}
nav li.lvl2{padding-left:14px;font-size:.85rem}
nav a{color:var(--muted);text-decoration:none}
nav a:hover{color:var(--accent);text-decoration:underline}
main{max-width:var(--max);margin:0 auto;padding:40px 24px 80px}
h2{font-family:system-ui,sans-serif;font-size:1.6rem;color:var(--accent);border-bottom:2px solid var(--line);padding-bottom:6px;margin-top:56px;scroll-margin-top:50px}
h3{font-family:system-ui,sans-serif;font-size:1.15rem;color:#33415e;margin-top:32px}
table{border-collapse:collapse;width:100%;margin:20px 0;font-size:.92rem;font-family:system-ui,sans-serif}
th,td{border:1px solid var(--line);padding:8px 10px;text-align:left;vertical-align:top}
th{background:#f0f3f8}
ul{padding-left:24px}
li{margin:6px 0}
footer{border-top:1px solid var(--line);color:var(--muted);text-align:center;padding:24px;font-size:.85rem;font-family:system-ui,sans-serif}
@media(max-width:720px){nav ul{columns:1}header.hero h1{font-size:1.7rem}}"""


def build(blocks):
    # The first h1 is the paper title, the first h2 the subtitle; both go to the hero.
    title = next(c for k, c in blocks if k == "h1")
    subtitle = next(c for k, c in blocks if k == "h2")

    body = []
    toc = []
    sid = 0
    seen_hero = {"h1": False, "h2": False}
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
        elif kind == "h3":
            body.append(f"<h3>{html.escape(content)}</h3>")
        elif kind == "li":
            body.append(f"<li>{content}</li>")
        elif kind == "p":
            body.append(f"<p>{content}</p>")
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
<header class="hero"><h1>{html.escape(title)}</h1><p>{html.escape(subtitle)}</p></header>
<nav><div class="inner"><details open><summary>Table of contents</summary><ul>{toc_html}</ul></details></div></nav>
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
