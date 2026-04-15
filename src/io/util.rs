use super::{ErrorKind, Read, Result, Write};

// =============================================================================
// copy

/// Copies the entire contents of a reader into a writer.
///
/// This function will continuously read data from `reader` and then
/// write it into `writer` in a streaming fashion until `reader`
/// returns EOF.
///
/// On success, returns the total number of bytes copied.
///
/// # Errors
///
/// This function will return an error immediately if any call to
/// [`read`] or [`write`] returns an error. All instances of
/// [`ErrorKind::Interrupted`] are handled and the operation is retried.
///
/// [`read`]: Read::read
/// [`write`]: Write::write
///
/// # Examples
///
/// ```
/// use core3::io;
///
/// let mut reader: &[u8] = b"hello";
/// let mut writer: Vec<u8> = vec![];
///
/// io::copy(&mut reader, &mut writer).unwrap();
///
/// assert_eq!(&writer, b"hello");
/// ```
pub fn copy<R: Read + ?Sized, W: Write + ?Sized>(
    reader: &mut R,
    writer: &mut W,
) -> Result<u64> {
    let mut buf = [0u8; 8192];
    let mut total = 0u64;
    loop {
        let n = match reader.read(&mut buf) {
            Ok(0) => return Ok(total),
            Ok(n) => n,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..n])?;
        total += n as u64;
    }
}

// =============================================================================
// empty

/// Creates a value that is always at EOF for reads, and ignores all data for writes.
///
/// All calls to [`read`] on the returned instance will return [`Ok(0)`].
///
/// [`read`]: Read::read
/// [`Ok(0)`]: Ok
///
/// # Examples
///
/// ```
/// use core3::io::{self, Read};
///
/// let mut buf = [0u8; 16];
/// let n = io::empty().read(&mut buf).unwrap();
/// assert_eq!(n, 0);
/// ```
pub fn empty() -> Empty {
    Empty
}

/// A reader which is always at EOF.
///
/// Created by the [`empty`] function.
#[derive(Clone, Copy, Debug, Default)]
pub struct Empty;

impl Read for Empty {
    #[inline]
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        Ok(0)
    }
}

// =============================================================================
// sink

/// Creates a writer which will successfully consume all data.
///
/// All calls to [`write`] on the returned instance will return `Ok(buf.len())`
/// and the contents of the buffer will not be inspected.
///
/// [`write`]: Write::write
///
/// # Examples
///
/// ```
/// use core3::io::{self, Write};
///
/// let n = io::sink().write(b"anything").unwrap();
/// assert_eq!(n, 8);
/// ```
pub fn sink() -> Sink {
    Sink
}

/// A writer which consumes and discards all data.
///
/// Created by the [`sink`] function.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sink;

impl Write for Sink {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    #[inline]
    fn write_all(&mut self, _buf: &[u8]) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

// =============================================================================
// repeat

/// Creates a reader that endlessly repeats a single byte.
///
/// All calls to [`read`] on the returned instance will fill the buffer
/// with `byte` and return `Ok(buf.len())`.
///
/// [`read`]: Read::read
///
/// # Examples
///
/// ```
/// use core3::io::{self, Read};
///
/// let mut buf = [0u8; 4];
/// io::repeat(0xAB).read_exact(&mut buf).unwrap();
/// assert_eq!(buf, [0xAB; 4]);
/// ```
pub fn repeat(byte: u8) -> Repeat {
    Repeat { byte }
}

/// A reader that endlessly repeats a single byte.
///
/// Created by the [`repeat`] function.
#[derive(Clone, Copy, Debug)]
pub struct Repeat {
    byte: u8,
}

impl Read for Repeat {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        buf.fill(self.byte);
        Ok(buf.len())
    }
}
