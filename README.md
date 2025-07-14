# rat: A Custom File Archiver Inspired by `tar`

`rat` (Rust Archiver Tool) is a command-line utility written in Rust that creates and extracts custom archive files with the `.rat` extension. It is inspired by the functionality of the Unix `tar` command and supports regular files, symbolic links, and hard links.

---

## Table of Contents

* [Features](#features)
* [Installation](#installation)
* [Usage](#usage)

  * [Creating an Archive](#creating-an-archive)
  * [Extracting an Archive](#extracting-an-archive)
* [Archive Format](#archive-format)
* [File Type Handling](#file-type-handling)
* [Technical Design](#technical-design)
* [Limitations](#limitations)
* [Future Work](#future-work)
* [License](#license)

---

## Features

* Archive multiple files and directories into a `.rat` file
* Handles the following file types:

  * Regular files
  * Symbolic links
  * Hard links
* Prevents duplicate file archiving using inode/device identification
* Preserves key metadata: inode, device ID, file size, permissions, and modification time
* Binary serialization using `bincode`
* Structured and extensible file format for future features

---

## Installation

Ensure that you have Rust and Cargo installed. Clone the repository and build the binary:

```bash
git clone https://github.com/yourusername/rat.git
cd rat
cargo build --release
```

The compiled binary will be available in `target/release/rat`.

---

## Usage

### Creating an Archive

```bash
rat create -f archive_name.rat -p file1.txt dir/ link_to_file
```

Options:

* `-f`, `--name`: Name of the archive file to create
* `-p`, `--paths`: List of files and directories to include in the archive

### Extracting an Archive

```bash
rat extract -f archive_name.rat --output-dir extracted/
```

Options:

* `-f`, `--name`: Name of the archive to extract from
* `--output-dir`: Target directory for extracted files

---

## Archive Format

Each entry in the `.rat` file is encoded as a struct:

```rust
enum FileType {
    RegFile,
    SymbFile,
    HrdLnkFile,
    CharDev,
    BlocDev
}

struct RatEntryFile {
    _path: String,
    _content: Option<Vec<u8>>,
    _size: u64,
    _target: Option<String>,
    _inode: u64,
    _dev: u64,
    _mode: u32,
    _m_time: u64,
    _type: FileType
}
```

Directories can be encoded with:

```rust
struct RatEntryDir {
    _path: String,
    _files: Vec<RatEntryFile>
}
```

Serialization is done with `bincode::encode_into_writer`.

---

## File Type Handling

* **Regular files**: Serialized with their binary content
* **Symbolic links**: Store link target path; no content saved
* **Hard links**: Avoid duplicating content using a `(dev, inode)` map
* **Duplicate paths**: Skipped if already seen (canonical form)

Detection of file type is done via `symlink_metadata`.

---

## Technical Design

* Written entirely in Rust
* Uses `BufWriter` for efficient writing
* Serializes metadata and content using `bincode`
* Metadata collected via `fs::symlink_metadata`
* Content read using `fs::read` and stored as `Vec<u8>`

Example serialization:

```rust
encode_into_writer(entry, &mut writer, Config::default())?;
```

---

## Limitations

* No compression (future feature)
* POSIX-specific: Uses Unix-only traits (e.g., `MetadataExt`)
* Extraction logic not fully implemented
* Does not preserve file permissions or ownership during extraction

---

## Future Work

* Add compression support (gzip, zstd)
* Implement extraction logic
* Cross-platform support (Windows, macOS)
* Restore permissions and timestamps after extraction
* Add integrity checks (hashing)
* Support for extended file types (sockets, pipes)

