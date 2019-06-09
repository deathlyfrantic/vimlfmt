## viml-parser-rs

A parser for VimL, written in Rust.

This started life as a direct port of [the vim-jp
parser](https://github.com/vim-jp/vim-vimlparser) and grew from there. As of
commit 4f09a92, output of this parser exactly matched output of the original for
every .vim file in my ~/.config/nvim/ directory (45-50 plugins). Since that
point, output no longer matches, because I have changed some behavior (e.g. this
continues parsing after a top-level `finish`) and fixed some bugs.

### Usage

There are two exposed functions: `parse_lines()` and `parse_file()`. Both will
return a `Node` enum, the specific variant of which identifies the node type,
and contains an inner struct with data specific to that node. See the
documentation for further details.

### License

BSD 2-clause
