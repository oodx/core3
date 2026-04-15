use core3::io::{Cursor, Error, ErrorKind, Read, Seek, SeekFrom, Write};

#[test]
fn cursor_read() {
    let mut buf = Cursor::new(b"hello" as &[u8]);
    let mut out = [0u8; 5];
    buf.read_exact(&mut out).unwrap();
    assert_eq!(&out, b"hello");
}

#[test]
fn cursor_write_slice() {
    let mut backing = [0u8; 10];
    let mut buf = Cursor::new(&mut backing[..]);
    buf.write_all(b"hello").unwrap();
    assert_eq!(&backing[..5], b"hello");
}

#[test]
fn cursor_seek() {
    let mut buf = Cursor::new(b"\x01\x02\x03\x04\x05" as &[u8]);
    buf.seek(SeekFrom::Start(3)).unwrap();
    let mut out = [0u8; 1];
    buf.read(&mut out).unwrap();
    assert_eq!(out[0], 4);
}

#[test]
fn cursor_seek_from_end() {
    let mut buf = Cursor::new(b"abcdef" as &[u8]);
    buf.seek(SeekFrom::End(-2)).unwrap();
    let mut out = [0u8; 2];
    buf.read_exact(&mut out).unwrap();
    assert_eq!(&out, b"ef");
}

#[test]
fn slice_read() {
    let data: &[u8] = b"world";
    let mut out = [0u8; 5];
    let mut reader = data;
    reader.read_exact(&mut out).unwrap();
    assert_eq!(&out, b"world");
}

#[test]
fn slice_write() {
    let mut buf = [0u8; 5];
    let mut writer = &mut buf[..];
    writer.write_all(b"hello").unwrap();
    assert_eq!(&buf, b"hello");
}

#[test]
fn error_kind_roundtrip() {
    let err = Error::from(ErrorKind::NotFound);
    assert_eq!(err.kind(), ErrorKind::NotFound);
    assert_eq!(format!("{}", err), "entity not found");
}

#[test]
fn error_custom_message() {
    let err = Error::new(ErrorKind::Other, "something broke");
    assert_eq!(err.kind(), ErrorKind::Other);
    assert_eq!(format!("{}", err), "something broke");
}

#[test]
fn chain_read() {
    let a: &[u8] = b"hello ";
    let b: &[u8] = b"world";
    let mut chained = a.chain(b);
    let mut out = vec![0u8; 11];
    chained.read_exact(&mut out).unwrap();
    assert_eq!(&out, b"hello world");
}

#[test]
fn take_limits_bytes() {
    let data: &[u8] = b"hello world";
    let mut limited = data.take(5);
    let mut out = vec![0u8; 5];
    limited.read_exact(&mut out).unwrap();
    assert_eq!(&out, b"hello");
}

#[test]
fn bytes_iterator() {
    let data: &[u8] = b"abc";
    let reader = data;
    let bytes: Vec<u8> = reader.bytes().map(|b| b.unwrap()).collect();
    assert_eq!(bytes, vec![b'a', b'b', b'c']);
}

#[cfg(not(feature = "std"))]
mod no_std_tests {
    use core3::io::ErrorKind;

    #[test]
    #[allow(deprecated)]
    fn initializer_compat() {
        use core3::io::Initializer;

        let init = Initializer::zeroing();
        assert!(init.should_initialize());

        let mut buf = [0xFFu8; 4];
        init.initialize(&mut buf);
        assert_eq!(buf, [0, 0, 0, 0]);

        let nop = unsafe { Initializer::nop() };
        assert!(!nop.should_initialize());

        let mut buf2 = [0xFFu8; 4];
        nop.initialize(&mut buf2);
        assert_eq!(buf2, [0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[cfg(feature = "utils")]
    mod utils {
        use super::ErrorKind;
        use core3::io::{BufRead, Cursor, Read};

        #[test]
        fn read_to_string_valid() {
            let mut reader: &[u8] = b"hello";
            let mut out = String::new();
            let n = reader.read_to_string(&mut out).unwrap();
            assert_eq!(n, 5);
            assert_eq!(out, "hello");
        }

        #[test]
        fn read_to_string_invalid_utf8() {
            let mut reader: &[u8] = &[0xFF, 0xFE];
            let mut out = String::new();
            let err = reader.read_to_string(&mut out).unwrap_err();
            assert_eq!(err.kind(), ErrorKind::InvalidData);
        }

        #[test]
        fn read_until_delimiter() {
            let mut cursor = Cursor::new(b"hello\nworld\n" as &[u8]);
            let mut buf = vec![];
            let n = cursor.read_until(b'\n', &mut buf).unwrap();
            assert_eq!(n, 6);
            assert_eq!(&buf, b"hello\n");
        }

        #[test]
        fn read_until_eof() {
            let mut cursor = Cursor::new(b"no newline" as &[u8]);
            let mut buf = vec![];
            let n = cursor.read_until(b'\n', &mut buf).unwrap();
            assert_eq!(n, 10);
            assert_eq!(&buf, b"no newline");
        }

        #[test]
        fn read_line_basic() {
            let mut cursor = Cursor::new(b"first\nsecond\n" as &[u8]);
            let mut line = String::new();
            let n = cursor.read_line(&mut line).unwrap();
            assert_eq!(n, 6);
            assert_eq!(line, "first\n");
        }

        #[test]
        fn read_line_no_newline() {
            let mut cursor = Cursor::new(b"only line" as &[u8]);
            let mut line = String::new();
            let n = cursor.read_line(&mut line).unwrap();
            assert_eq!(n, 9);
            assert_eq!(line, "only line");
        }
    }
}

#[cfg(feature = "utils")]
#[test]
fn copy_reader_to_writer() {
    use core3::io;
    let mut reader: &[u8] = b"hello world";
    let mut writer: Vec<u8> = vec![];
    let n = io::copy(&mut reader, &mut writer).unwrap();
    assert_eq!(n, 11);
    assert_eq!(&writer, b"hello world");
}

#[cfg(feature = "utils")]
#[test]
fn copy_empty_source() {
    use core3::io;
    let mut reader: &[u8] = b"";
    let mut writer: Vec<u8> = vec![];
    let n = io::copy(&mut reader, &mut writer).unwrap();
    assert_eq!(n, 0);
    assert!(writer.is_empty());
}

#[cfg(feature = "utils")]
#[test]
fn empty_reader() {
    use core3::io::{self, Read};
    let mut buf = [0xFFu8; 16];
    let n = io::empty().read(&mut buf).unwrap();
    assert_eq!(n, 0);
}

#[cfg(feature = "utils")]
#[test]
fn sink_writer() {
    use core3::io::{self, Write};
    let mut s = io::sink();
    assert_eq!(s.write(b"discard me").unwrap(), 10);
    s.write_all(b"also this").unwrap();
    s.flush().unwrap();
}

#[cfg(feature = "utils")]
#[test]
fn repeat_reader() {
    use core3::io::{self, Read};
    let mut buf = [0u8; 8];
    io::repeat(0x42).read_exact(&mut buf).unwrap();
    assert_eq!(buf, [0x42; 8]);
}

// Tests that only work in std mode (std::io passthrough)
#[cfg(feature = "std")]
mod std_tests {
    use core3::io::{BufRead, Cursor, Read, Write};

    #[test]
    fn vec_write() {
        let mut buf = Vec::new();
        buf.write_all(b"hello").unwrap();
        assert_eq!(buf, b"hello");
    }

    #[test]
    fn cursor_vec_write() {
        let mut buf = Cursor::new(vec![0u8; 10]);
        buf.write_all(b"hello").unwrap();
        buf.set_position(0);
        let mut out = [0u8; 5];
        buf.read_exact(&mut out).unwrap();
        assert_eq!(&out, b"hello");
    }

    #[test]
    fn buf_read_lines() {
        let data = Cursor::new(b"line1\nline2\nline3");
        let lines: Vec<_> = data.lines().collect::<Result<_, _>>().unwrap();
        assert_eq!(lines, vec!["line1", "line2", "line3"]);
    }

    #[test]
    fn read_to_string() {
        let data: &[u8] = b"hello";
        let mut reader = data;
        let mut out = String::new();
        reader.read_to_string(&mut out).unwrap();
        assert_eq!(out, "hello");
    }
}
