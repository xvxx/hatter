## v0.1.4

- Hatter maps now perserve insertion order, unlike Rust but like
  Ruby/JS/Lua.
- More [builtins]: `pop()` for lists.
- Finally added `set_index` operations for List, Map, and Object,
  both `a[1] = 2` and `map.thing = true` forms.
- Fixed a bug in `a[-1]` operations.

[builtins]: https://docs.rs/hatter/latest/hatter/builtin/index.html

## v0.1.3

- Lists can now be indexed with negative numbers, eg `list[-1]` is the last element.
- Added `contains?()`, `split()`, `count()`, and `push()` builtins.
- For a full list of builtins, see https://docs.rs/hatter/latest/hatter/builtin/index.html

## v0.1.2

- Added `puts()` builtin to print with trailing newline.
- Made `print()` print without newline.
- Added `line_and_col()` to public API to get file location information
  from an error.
- Tag body expressions that begin with a number followed by a word are
  now treated as implicit text.
- trait Object now requires typename() fn.

## v0.1.1

- Added `hatter check` CLI command.
- Allow indented text in tag bodies. (Still not perfect.)
- Bugfix on unexpected EOF.

## v0.1.0

- First release. Pre-pre-pre alpha.
