# Contributing to the Existenz White Paper

Thank you for engaging with this proposal. The document deliberately states
its own weaknesses (see Section 11, "Known Limits and Structural
Weaknesses"). Contributions that identify new weaknesses, correct facts, or
challenge the design are as welcome as improvements.

## How to propose a change

1. Fork the repository.
2. Edit `docs/existenz_wp.md` (never `index.html`, which is generated).
3. Rebuild locally to check the rendering:
   `python3 scripts/build_site.py docs/existenz_wp.md index.html`
4. Open a pull request describing what you changed and, if you are
   challenging the design, why the current text is wrong.

A reviewer will merge the pull request; the automated workflow then rebuilds
the published site.

## Types of contribution

- Corrections: factual errors, arithmetic mistakes, broken references.
- Critiques: new structural weaknesses for Section 11 or an existing annex.
- Economic, cryptographic, or governance analysis, ideally with sources.
- Translations (coordinate first by opening an issue).

## Style

- The document is written in English.
- Keep the honest tone: weaknesses are stated, not hidden.
- Prefer plain, precise prose; no marketing language.

## Issues

Use GitHub Issues for questions, objections, and error reports before
writing a full pull request. Label objections as `critique`.
