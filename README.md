# oxidex core3


~ Made this for my own needs but anyone who wants it can use it. 

A `no_std` replacement for [`core2`](https://crates.io/crates/core2). Provides the essential `std::io` traits (`Read`, `Write`, `Seek`, `BufRead`, `Cursor`) for `no_std` environments.


Forked from core2 v0.4.0. Modernized for current Rust:

- **Removed** the `error::Error` trait polyfill (stable in `core` since Rust 1.81)
- **Removed** nightly-only items (`BufReader`, `BufWriter`, `LineWriter`)
- **MSRV: 1.81**
- Single dependency: [`memchr`](https://crates.io/crates/memchr)


## Usage

```toml
[dependencies]
core3 = { version = "0.1", default-features = false }
```

```rust
use core3::io::{Read, Write, Cursor};
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `std`   | ✅      | Re-exports `std::io` directly |
| `alloc` | via std | Enables `Vec<u8>` Write impl and allocating Read methods |

For `no_std` without alloc:

```toml
core3 = { version = "0.1", default-features = false }
```

For `no_std` with alloc:

```toml
core3 = { version = "0.1", default-features = false, features = ["alloc"] }
```

## API

**Traits:** `Read`, `Write`, `Seek`, `BufRead`

**Structs:** `Cursor`, `Bytes`, `Chain`, `Take`, `Empty`, `Repeat`, `Sink`

**Enums:** `SeekFrom`, `ErrorKind`

**Types:** `Error`, `Result`

**Free functions:** `copy`, `empty`, `repeat`, `sink`

### Utilities

These mirror their `std::io` equivalents and work in `no_std`:

| Function | Needs `alloc` | Description |
|----------|:---:|-------------|
| `copy(reader, writer)` | no | Stream all bytes from a reader into a writer |
| `empty()` | no | Returns a reader that is always at EOF |
| `sink()` | no | Returns a writer that discards all data |
| `repeat(byte)` | no | Returns a reader that endlessly yields one byte |

### Additional trait methods (beyond core2)

| Method | Needs `alloc` | Description |
|--------|:---:|-------------|
| `Read::read_to_string()` | yes | Read all bytes into a `String`, errors on invalid UTF-8 |
| `BufRead::read_until(byte, buf)` | yes | Read until a delimiter byte or EOF |
| `BufRead::read_line(buf)` | yes | Read until newline into a `String` |

## Migrating from core2

### `Initializer` is deprecated

The `Initializer` type and `Read::initializer()` method from core2 are preserved
but deprecated. These mirrored an unstable std API (`#![feature(read_initializer)]`)
that was never stabilized. Zero-initialize buffers before calling `read()` instead.

Replace the dependency:

```toml
# Before
core2 = { version = "0.4", default-features = false }
# After
core3 = { version = "0.1", default-features = false }
```

Then update imports:

```rust
// Before
use core2::io::{Read, Write};

// After
use core3::io::{Read, Write};
```

## Why does this exist?

In April 2026 all versions of core2 were yanked. `core::error::Error` was stabilized in Rust 1.81 (September 2024), so the error polyfill is no longer needed. The I/O traits (`Read`, `Write`, `Seek`, `BufRead`, `Cursor`) still have no equivalent in `core`.

This crate extracts just the I/O components that the ecosystem still needs.

## Alternatives

If core3 isn't the right fit, consider [`no-std-io2`](https://github.com/wcampbell0x2a/no-std-io2), a maintained fork of core2.

## License

Apache-2.0 OR MIT (same as the original core2)
