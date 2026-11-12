# Contributing to the Existenz White Paper

Thank you for engaging with this proposal. The document deliberately states
its own weaknesses (see Section 11, "Known Limits and Structural
Weaknesses"). Contributions that identify new weaknesses, correct facts, or
challenge the design are as welcome as improvements.

You do not need to be a developer, and you do not need to install anything.

## First: say it in Discussions

The primary place to contribute today is the
[Discussions](https://github.com/davidbenoitbetmalle/existenz/discussions)
tab. If you have read part of the paper and you disagree, or you think
something is wrong or unclear, open a discussion:

- A paragraph with your objection, and the section or annex you are
  challenging.
- Arithmetic, field data or a citation when you have them: an objection
  with a number in it is worth a page of opinion.
- French is explicitly welcome; maintainers will summarize threads that
  change the design into English in the paper.

Start with the pinned discussion
["Where this paper is most likely to be wrong: three open questions"](https://github.com/davidbenoitbetmalle/existenz/discussions/2)
if your objection fits one of them.

## Second: propose a text change

When a discussion converges on a concrete change to the paper, the natural
next step is a text proposal:

1. Open `docs/existenz_wp.md` in this repository.
2. Click the pencil icon at the top right of the file.
3. Edit the text like any document. GitHub handles everything technical:
   it creates the fork and the pull request for you automatically.
4. Click "Propose changes" and describe in one or two sentences what you
   changed and why, referencing the discussion it came from.

Never edit `index.html` directly: it is generated from the markdown source.

## Issues

Use Issues for specific, trackable items: factual errors, arithmetic
mistakes, broken references. One issue per problem, so each can be closed
when fixed. Label objections as `critique`.

## Types of contribution

- Corrections: factual errors, arithmetic mistakes, broken references.
- Critiques: new structural weaknesses for Section 11 or an existing annex.
- Economic, cryptographic, or governance analysis, ideally with sources.
- Translations (coordinate first by opening an issue).

## Style

- The document is written in English.
- Keep the honest tone: weaknesses are stated, not hidden.
- Prefer plain, precise prose; no marketing language.

## For reviewers

When merging a contribution that changes the design, update Section 11 or
the relevant annex rather than only the section the contributor edited: the
promise of this paper is that its weaknesses stay stated, not hidden.
