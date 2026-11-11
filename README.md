# Existenz: a Fair Global Currency

This repository holds the source and the published website of the Existenz
(EXZ) white paper: a protocol-level proposal for a fair global virtual
currency that recognizes every human existence, deliberately published with
its known limits stated openly.

The white paper is published as a single-page website on GitHub Pages:
`https://<owner>.github.io/existenz/`

## Repository structure

```
docs/existenz_wp.md       The white paper source (markdown)
scripts/build_site.py     Builds the single-page website from the source
index.html                Generated output, published by GitHub Pages
.github/workflows/        CI: rebuilds the site when the source changes
```

`index.html` is generated. Do not edit it by hand: edit the markdown source
instead.

## Build the site locally

Python 3 is the only requirement (standard library only):

```
python3 scripts/build_site.py docs/existenz_wp.md index.html
```

## Automated builds

Every push to `main` that changes the markdown source (or the build script)
triggers the `build-site` workflow, which regenerates `index.html` and
commits it back to the branch. GitHub Pages then serves the updated site.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). The project particularly welcomes
criticism: Section 11 of the white paper exists precisely because the
authors believe a proposal like this must be attacked before it can be
trusted.
