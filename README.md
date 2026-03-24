# Word Ninja Rust Lua

[Wordninja Rust](https://github.com/chengyuhui/wordninja-rs) lua native module.

[![MIT licensed][mit-badge]][mit-url] [![AUR][aur-badge]][aur-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
[aur-badge]: https://img.shields.io/aur/version/wordninja-rs-lua-git.svg
[aur-url]: https://aur.archlinux.org/packages/wordninja-rs-lua-git

**You MUST rename target file to `wordninja.so` (or `wordninja.dll` if use Windows) before require it in Lua.**

## Dictionary

Word segmentation requires a word-frequency dictionary to determine the optimal way to split concatenated text. This library ships with a built-in English dictionary (126,136 words compiled into the binary via the `wordninja` crate), so no external dictionary file is needed for English.

If you need to segment text using a custom word list (e.g. for domain-specific terms), you can provide a newline-separated word list to `split_with_model`. Words should be ordered from most frequent to least frequent.

## How to use

Depends `rust` `lua5.4`/`lua5.3`

``` shell
$ cargo build --release --features "lua54"
$ cp target/release/libwordninja_rs_lua.so wordninja.so

$ lua5.4 -e 'print(require("wordninja").split("iloverust"))'
i love rust
```

### Using a custom dictionary

``` lua
local wordninja = require("wordninja")

-- Load a custom word list (one word per line, most frequent first)
local f = io.open("my_words.txt", "r")
local dict = f:read("*a")
f:close()

print(wordninja.split_with_model("customtext", dict))
```
