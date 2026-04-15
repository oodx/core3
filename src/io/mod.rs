// Copyright (c) core2 contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod cursor;
mod error;
mod impls;
mod traits;
#[cfg(feature = "utils")]
mod util;

pub use cursor::Cursor;
pub use error::{Error, ErrorKind, Result};
#[allow(deprecated)]
pub use traits::Initializer;
pub use traits::{BufRead, Bytes, Chain, Read, Seek, SeekFrom, Take, Write};
#[cfg(feature = "utils")]
pub use util::{copy, empty, repeat, sink, Empty, Repeat, Sink};
