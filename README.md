# isi
<div align="center">
  <img width="127" height="127" alt="Frame 11" src="https://github.com/user-attachments/assets/a4544da0-ec29-4bfb-ac92-26dd442e01c8" />
</div>


A minimalist recreation of Git's core mechanics, written in Rust. `isi` implements the fundamental building blocks of version control: content-addressable object storage, a staging index, and diff between stored objects.

## How it works

`isi` mirrors Git's internal architecture at its simplest:

- **Objects** are stored in `.isi/objects/` compressed with zlib, addressed by their SHA-1 hash (first 2 chars as directory, remaining 38 as filename).
- **Blobs** store file content prefixed with a `blob <size>\0` header — identical to Git's blob format.
- **Trees** store directory snapshots as binary entries (`<mode> <name>\0<hash_bytes>`).
- **Index** (`.isi/index`) is a plain-text staging area that maps `hash → relative path`, persisting what has been added across commands.
- **`.isiignore`** controls which files and directories are excluded from `add`.

Since `isi` uses the same SHA-1 + zlib format as Git, objects stored by `isi` can be read directly by `git cat-file`.

## Installation

```bash
git clone https://github.com/glrmrissi/isi
cd git-be-like
cargo install --path .
```

The `isi` binary is placed in `~/.cargo/bin/` and works from any directory.

## Commands

### `isi init`

Initializes a new repository in the current directory by creating `.isi/objects/` and `.isi/refs/`.

```bash
isi init
# .isi repository initialized successfully!
```

---

### `isi add <path>`

Hashes and compresses a file or directory into the object store, then records the result in `.isi/index`.

```bash
# Add a single file
isi add src/main.rs

# Add everything (respects .isiignore)
isi add .
```

Output: `<sha1-hash>  <path-relative-to-repo-root>`

When run from a subdirectory, `isi` walks up to find the repo root, so paths in the index are always relative to the root.

---

### `isi cat-file -p <hash>`

Prints the raw content of any stored object by its SHA-1 hash.

```bash
isi cat-file -p ab92bf26554b580632ffd37ff6231a43a6737e89
```

```
--- Content Start ---
fn main() { ... }
--- Content End ---
```

---

### `isi diff <old-hash> <new-hash>`

Shows a line-level diff between two stored objects. Removed lines are red (`-`), added lines are green (`+`).

```bash
isi diff ab92bf26 e3647404
```

### `isi diff` No arguments

Shows all the difference between the saved index and the index on the local disk

```bash
isi diff
```

```
--- Diff between ab92bf2 and e364740 ---
  fn main() {
-     println!("hello");
+     println!("world");
  }
```

---

## .isiignore

Place a `.isiignore` file at the repo root to exclude files and directories from `isi add`. The syntax is a subset of `.gitignore`:

| Pattern      | Behavior                                              |
|--------------|-------------------------------------------------------|
| `target/`    | Ignore directories named `target` (trailing `/`)      |
| `*.log`      | Ignore files matching the glob anywhere in the tree   |
| `Cargo.lock` | Ignore by exact filename at any depth                 |
| `src/secret` | Ignore a path anchored to the repo root (contains `/`)|
| `# comment`  | Line comment, ignored                                 |
| *(blank)*    | Ignored                                               |

`*` matches any sequence of characters that does not cross a directory boundary.

Example `.isiignore`:

```
# Build artifacts
target/

# Auto-generated lock file
Cargo.lock

# Temporary and system files
*.tmp
*.log
.DS_Store

# Secrets
.env
*.key
*.pem
```

## Dependencies

| Crate    | Purpose                        |
|----------|--------------------------------|
| `clap`   | CLI argument parsing           |
| `sha1`   | SHA-1 hashing for object IDs   |
| `flate2` | zlib compression/decompression |
| `hex`    | Hex encoding for tree entries  |
| `diff`   | Line-level diff algorithm      |
