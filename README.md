# rgrep

A small grep-like command-line tool written in Rust.

## Usage

```text
cargo run -- [OPTIONS] PATTERN PATH...
```

The first positional argument is the pattern. The remaining arguments are file or directory paths. Directories require `-r`.

Options:

- `-i` Match without regard to letter case.
- `-n` Show line numbers.
- `-r` Search directories and their subdirectories.
- `-v` Show lines that do not match the pattern.
- `-c` Show the number of matching lines per file, along with the matching lines.

Examples:

```text
cargo run -- -i -n error sample.txt sample.md
cargo run -- -r TODO src
```

Files that cannot be read as UTF-8 are skipped.
