# Existenz: a Fair Global Currency

## Start here

**[Where this paper is most likely to be wrong: three open questions](https://github.com/davidbenoitbetmalle/existenz/discussions/2)**
(planetary-scale identity, solvability arithmetic, melting parameters that
have never been measured)

This project's first need is not praise but objections. Read the
[white paper](https://davidbenoitbetmalle.github.io/existenz/), then attack
it in the discussion above.

This repository holds the source and the published website of the Existenz
(EXZ) white paper: a protocol-level proposal for a fair global virtual
currency that recognizes every human existence, deliberately published with
its known limits stated openly.

It is also the home of the future code: the reference implementation of the
protocol will be built here, in the open, alongside the specification it
implements (see below).

The white paper is published as a single-page website on GitHub Pages:
[`https://<owner>.github.io/existenz/`](https://davidbenoitbetmalle.github.io/existenz/)

## Repository structure

```
docs/existenz_wp.md       The white paper source (markdown)
scripts/build_site.py     Builds the single-page website from the source
index.html                Generated output, published by GitHub Pages
.github/workflows/        CI: rebuilds the site when the source changes
```

The implementation directories will be added when the first code exists, for
example:

```
prototype/                First executable prototypes of protocol components
src/                      Reference implementation of the protocol
tests/                    Specifications-as-code and test suites
```

`index.html` is generated. Do not edit it by hand: edit the markdown source
instead.

## The repository hosts the specification and the code

This repository is deliberately both the paper and the future build. The
intended sequence, mirroring the white paper's roadmap (Section 10):

1. Specification first (today): the markdown source is the single point of
   truth; changes go through pull requests.
2. Prototypes next: executable models of isolated components (melting
   mechanics, the day-chain, the BNB index sampling) will live under
   `prototype/`, so that parameters can be measured rather than reasoned.
3. Reference implementation later: the protocol components specified in
   Annexes A to F (identity, offline verification, merchant settlement)
   will be implemented under `src/` with tests under `tests/`, reviewed
   against the specification.

Nothing in `prototype/` or `src/` will be considered production until the
corresponding annex has been challenged in Discussions or Issues first.

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
