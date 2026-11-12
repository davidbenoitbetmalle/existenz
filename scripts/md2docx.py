#!/usr/bin/env python3
"""Convert the white paper markdown subset to a standalone DOCX.

Dependency-free (standard library only): parses the same markdown subset as
scripts/build_site.py (h1/h2/h3 headings, paragraphs, '- ' bullets, pipe
tables, **bold** and *italic* inline) and emits a valid .docx zip.
Usage: python3 md2docx.py docs/existenz_wp.md Existenz_White_Paper.docx
"""
import re
import sys
import zipfile
from datetime import datetime, timezone

W_NS = 'xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"'


def esc(t):
    return t.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def inline_runs(text):
    """Split on **bold** and *italic* spans, return (text, bold, italic) chunks."""
    spans = []
    pos = 0
    for m in re.finditer(r"\*\*([^*]+)\*\*|\*([^*]+)\*", text):
        if m.start() > pos:
            spans.append((text[pos:m.start()], False, False))
        if m.group(1) is not None:
            spans.append((m.group(1), True, False))
        else:
            spans.append((m.group(2), False, True))
        pos = m.end()
    if pos < len(text):
        spans.append((text[pos:], False, False))
    return spans


def run_xml(text, bold=False, italic=False):
    rpr = ""
    if bold:
        rpr += "<w:b/>"
    if italic:
        rpr += "<w:i/>"
    rpr = f"<w:rPr>{rpr}</w:rPr>" if rpr else ""
    preserve = ' xml:space="preserve"' if text != text.strip() else ""
    return f"<w:r>{rpr}<w:t{preserve}>{esc(text)}</w:t></w:r>"


def styled_run(text, bold=False, italic=False, size=None):
    rpr = ""
    if bold:
        rpr += "<w:b/>"
    if italic:
        rpr += "<w:i/>"
    if size:
        rpr += f'<w:sz w:val="{size}"/>'
    rpr = f"<w:rPr>{rpr}</w:rPr>" if rpr else ""
    return f"<w:r>{rpr}<w:t>{esc(text)}</w:t></w:r>"


def para_xml(text, style=None, bullet=False, cell=False):
    ppr = ""
    if style:
        ppr += f'<w:pStyle w:val="{style}"/>'
    if bullet:
        ppr += '<w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr>'
    if cell:
        ppr += "<w:spacing w:before=\"20\" w:after=\"20\"/>"
    ppr = f"<w:pPr>{ppr}</w:pPr>" if ppr else ""
    runs = "".join(run_xml(t, b, i) for t, b, i in inline_runs(text))
    return f"<w:p>{ppr}{runs}</w:p>"


def table_xml(rows):
    ncols = max(len(r) for r in rows)
    xml = (
        '<w:tbl><w:tblPr><w:tblStyle w:val="TableGrid"/>'
        '<w:tblW w:w="5000" w:type="pct"/>'
        '<w:tblBorders>'
        + "".join(
            f'<w:{s} w:val="single" w:sz="4" w:space="0" w:color="999999"/>'
            for s in ("top", "left", "bottom", "right", "insideH", "insideV")
        )
        + "</w:tblBorders></w:tblPr>"
    )
    for ri, row in enumerate(rows):
        xml += "<w:tr>"
        for ci in range(ncols):
            text = row[ci] if ci < len(row) else ""
            bold = ri == 0
            runs = (
                run_xml(text, bold=True)
                if bold
                else "".join(
                    run_xml(t, b, i) for t, b, i in inline_runs(text)
                )
            )
            shade = (
                '<w:shd w:val="clear" w:fill="F2F2F2"/>' if ri == 0 else ""
            )
            xml += (
                f"<w:tc><w:tcPr><w:tcW w:w=\"{5000 // max(ncols, 1)}\" "
                f'type="pct"/>{shade}</w:tcPr>'
                f'<w:p><w:pPr><w:spacing w:before="20" w:after="20"/></w:pPr>{runs}</w:p></w:tc>'
            )
        xml += "</w:tr>"
    xml += "</w:tbl>"
    return xml


def parse_markdown(text):
    """Yield blocks: ('h1'|'h2'|'h3'|'p'|'bullet'|'table', payload)."""
    lines = text.split("\n")
    i = 0
    blocks = []
    para_buf, table_buf = [], []

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
            blocks.append(("bullet", s[2:]))
        else:
            para_buf.append(s)
        i += 1
    flush_para()
    flush_table()
    return blocks


def build_document(blocks, title, subtitle, tagline):
    body = []
    body.append(
        '<w:p><w:pPr><w:jc w:val="center"/><w:spacing w:before="2400" '
        'w:after="240"/></w:pPr>'
        + styled_run(title, bold=True, size=56)
        + "</w:p>"
    )
    body.append(
        '<w:p><w:pPr><w:jc w:val="center"/><w:spacing w:after="120"/></w:pPr>'
        + styled_run(subtitle, bold=True, size=32)
        + "</w:p>"
    )
    body.append(
        '<w:p><w:pPr><w:jc w:val="center"/><w:spacing w:after="480"/></w:pPr>'
        + styled_run(tagline, italic=True)
        + "</w:p>"
    )
    for kind, payload in blocks:
        if kind in ("h1", "h2", "h3"):
            style = {"h1": "Heading1", "h2": "Heading2", "h3": "Heading3"}[kind]
            body.append(para_xml(payload, style=style))
        elif kind == "p":
            body.append(para_xml(payload))
        elif kind == "bullet":
            body.append(para_xml(payload, bullet=True))
        elif kind == "table":
            body.append(table_xml(payload))
    sect = (
        '<w:sectPr><w:pgSz w:w="11906" w:h="16838"/>'
        '<w:pgMar w:top="1418" w:right="1418" w:bottom="1418" '
        'w:left="1418"/></w:sectPr>'
    )
    return (
        f'<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        f'<w:document {W_NS}><w:body>' + "".join(body) + sect + "</w:body></w:document>"
    )


CONTENT_TYPES = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
<Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/>
<Override PartName="/word/numbering.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"/>
<Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
<Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
</Types>"""

RELS_ROOT = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
<Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>"""

RELS_DOC = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>
<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering" Target="numbering.xml"/>
</Relationships>"""

STYLES = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:docDefaults><w:rPrDefault><w:rPr>
<w:rFonts w:ascii="Calibri" w:hAnsi="Calibri" w:cs="Calibri"/>
<w:sz w:val="22"/><w:szCs w:val="22"/>
</w:rPr></w:rPrDefault>
<w:pPrDefault><w:pPr><w:spacing w:after="140" w:line="276" w:lineRule="auto"/></w:pPr></w:pPrDefault>
</w:docDefaults>
<w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/></w:style>
<w:style w:type="paragraph" w:styleId="Heading1"><w:name w:val="heading 1"/><w:basedOn w:val="Normal"/>
<w:pPr><w:keepNext/><w:spacing w:before="360" w:after="160"/>
<w:outlineLvl w:val="0"/><w:pBdr><w:bottom w:val="single" w:sz="6" w:space="4" w:color="D97706"/></w:pBdr></w:pPr>
<w:rPr><w:b/><w:sz w:val="32"/><w:color w:val="1F2937"/></w:rPr></w:style>
<w:style w:type="paragraph" w:styleId="Heading2"><w:name w:val="heading 2"/><w:basedOn w:val="Normal"/>
<w:pPr><w:keepNext/><w:spacing w:before="280" w:after="120"/><w:outlineLvl w:val="1"/></w:pPr>
<w:rPr><w:b/><w:sz w:val="26"/><w:color w:val="B45309"/></w:rPr></w:style>
<w:style w:type="paragraph" w:styleId="Heading3"><w:name w:val="heading 3"/><w:basedOn w:val="Normal"/>
<w:pPr><w:keepNext/><w:spacing w:before="200" w:after="100"/><w:outlineLvl w:val="2"/></w:pPr>
<w:rPr><w:b/><w:sz w:val="23"/><w:color w:val="374151"/></w:rPr></w:style>
<w:style w:type="table" w:styleId="TableGrid"><w:name w:val="Table Grid"/>
<w:tblPr><w:tblCellMar><w:top w:w="60" w:type="dxa"/><w:left w:w="120" w:type="dxa"/>
<w:bottom w:w="60" w:type="dxa"/><w:right w:w="120" w:type="dxa"/></w:tblCellMar></w:tblPr></w:style>
</w:styles>"""

NUMBERING = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:numbering xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
<w:abstractNum w:abstractNumId="0">
<w:lvl w:ilvl="0"><w:start w:val="1"/><w:numFmt w:val="bullet"/><w:lvlText w:val="&#8226;"/>
<w:lvlJc w:val="left"/><w:pPr><w:ind w:left="720" w:hanging="360"/></w:pPr>
<w:rPr><w:rFonts w:ascii="Symbol" w:hAnsi="Symbol"/></w:rPr></w:lvl>
</w:abstractNum>
<w:num w:numId="1"><w:abstractNumId w:val="0"/></w:num>
</w:numbering>"""


def core_xml(title):
    now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    return f"""<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
<dc:title>{esc(title)}</dc:title>
<dc:creator>David Benoit</dc:creator>
<cp:lastModifiedBy>David Benoit</cp:lastModifiedBy>
<dcterms:created xsi:type="dcterms:W3CDTF">{now}</dcterms:created>
<dcterms:modified xsi:type="dcterms:W3CDTF">{now}</dcterms:modified>
</cp:coreProperties>"""


APP_XML = """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties">
<Application>md2docx</Application></Properties>"""


def main():
    src, dst = sys.argv[1], sys.argv[2]
    text = open(src, encoding="utf-8").read()
    blocks = parse_markdown(text)

    # Title page pieces come from the first lines of the source.
    title = "The Existenz White Paper"
    subtitle = "A Fair Global Currency: Providing a Basic Human Right Through Virtual Currency"
    tagline = "A project by and for utopian and agnostic citizens of the world."

    # Skip the title block (first h1 + h2 + tagline paragraph) from the body.
    skipped = 0
    body_blocks = []
    for kind, payload in blocks:
        if skipped < 3 and kind in ("h1", "h2", "p"):
            if kind == "p" and payload.startswith("*"):
                skipped += 1
                continue
            if kind in ("h1", "h2"):
                skipped += 1
                continue
        body_blocks.append((kind, payload))

    doc = build_document(body_blocks, title, subtitle, tagline)
    with zipfile.ZipFile(dst, "w", zipfile.ZIP_DEFLATED) as z:
        z.writestr("[Content_Types].xml", CONTENT_TYPES)
        z.writestr("_rels/.rels", RELS_ROOT)
        z.writestr("word/document.xml", doc)
        z.writestr("word/_rels/document.xml.rels", RELS_DOC)
        z.writestr("word/styles.xml", STYLES)
        z.writestr("word/numbering.xml", NUMBERING)
        z.writestr("docProps/core.xml", core_xml(title))
        z.writestr("docProps/app.xml", APP_XML)
    print(f"Built {dst} ({len(doc)} bytes of document.xml, {len(body_blocks)} blocks)")


if __name__ == "__main__":
    main()
