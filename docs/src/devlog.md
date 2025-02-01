# Devlog

This is a document designed to capture thoughts and ideas as this project
progresses.

## Milestones

Currently the goal for `adamantite` is to implement a minimum viable Markdown
parser that can generate an AST for all sorts of fun manipulations. Then,
another crate (working name: `luminite`) can provide serialization from AST to
HTML. Both of these projects are designed to compose in a static site
generator, however, currently the concrete design of the site generator is not
well-thought out (and may end up being implemented on top of `trunk`).

We initially want to parse some of the most basic Markdown constructs while
staying mindful of future, more complex parsing requirements.

### Lexer progress

The very first thing we need is a lexer.

- [ ] Tokenize a markdown file of entirely plaintext into our lexer's data types
- [ ] Identify and tokenize basic constructs: header, italics, bolds, etc
- [ ] Identify and tokenize math
- [ ] Tokenize more complex structures: links, etc.

### AST generation progress

After lexing is complete, we need to begin AST generation. Currently nothing is
concretely planned here, except an initial test

- [ ] Create an AST from the most basic tokens (e.g. paragraphs of plaintext and headers)
