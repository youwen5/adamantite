# Design

The goal is to create a very simple Markdown parser that generates an AST which
can be manipulated. A separate crate will then handle serializing this AST into
various forms (first and foremost HTML, for static site generation). The
current aim is not to be the fastest or most compliant possible generator but
to create one flexible enough for my uses.

Currently the overall architecture needs to be designed in
[architecture](./architecture.md). We should first implement parsing of all
basic text objects (namely headers, images, links, italics, bold, etc), basic
frontmatter, and math to get a minimum working that can be dogfooded in e.g. a
personal website before implementing things like tables, etc.

## Math mode

I have been quite disappointed with math support in Markdown parsers and in
general it seems like a second class citizen. Math mode will be a major focus
in `adamantite` and the primary supported backend will be Typst, with
aspirations of supporting LaTeX. We will support inline math via dollar signs
and block math via the existing code block facilities. Currently the idea is:

```text
\```math-typ
blah blah
\```

\```math-tex
blah blah
\```
```
