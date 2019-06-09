## vimlfmt

A formatter for VimL code.

### Status

Very, very alpha. [Maybe abandoned?](#maybe-abandoned) Use at your own risk.

### Usage

See `vimlfmt --help`, but in general:

    vimlfmt < input.vim > output.vim

### Formatting Options

There aren't any. This formats VimL using two-space indents, tries to keep lines
shorter than 80 columns, and uses six spaces (three indents) for continued
lines. At this point in the project I do not want to add the complexity of
formatting options (VimL is already complicated enough).

### Limitations

- Primarily, most commands are parsed as generic `ExCmd` nodes, which include
  the arguments as a single raw string literal, so no formatting is done.
- There is no way to tell `vimlfmt` not to format part of a file.
- If some portion of the code doesn't parse, no formatting is done at all.

### Strategy

This formatter parses the input VimL into an abstract syntax tree and then
writes out every node of that tree. Other formatters I've looked at do more of
an "in-place" style of formatting, so I'm not sure if the way vimlfmt works is
optimal.

### Maybe abandoned?

Writing VimL isn't too bad (it's not great, but it's _mostly_ Ruby- or
Python-like). Parsing and formatting VimL is a **nightmare**. Its unending
inconsistency and unusual line continuation syntax make it much harder to format
than other languages (as far as I can tell).

There's still a lot to be done, but I'm pretty burned out on it for now. That
said, contributions are welcome! Having some help would be quite motivational.
It would be great if this project became something people could actually use.

### License

BSD 2-clause
