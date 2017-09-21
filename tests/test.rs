extern crate inflate;

fn get_test_file_data(name: &str) -> Vec<u8> {
    use std::fs::File;
    use std::io::Read;
    let mut input = Vec::new();
    let mut f = File::open(name).unwrap();

    f.read_to_end(&mut input).unwrap();
    input
}

#[test]
/// See https://github.com/PistonDevelopers/inflate/issues/14
fn issue_14() {
    let test_data = get_test_file_data("tests/issue_14.zlib");
    let res = inflate::inflate_bytes_zlib(&test_data);
    // This should fail as the file specifies code lengths that won't work.
    assert!(res.is_err());
}

#[test]
/// Another input that produce invalid code lengths.
fn issue_16() {
    let data = b"M\xff\xffM*\xad\xad\xad\xad\xad\xad\xad\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xcd\xad\xad\xad\xad\xad\xad\xad\xad\xad\xad\xad\xadMCMMMM\x00\x00\x00\x00\x00\x00\x00\x00\x00!\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xd9\xcb!\x0a";
    let mut stream = inflate::InflateStream::new();
    let res = stream.update(data);
    assert!(res.is_err());
}

#[test]
/// Similar to 16 but with CLENS being invalid.
fn issue_17() {
    let data = b"\xdd\xff\xff*M\x94ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\x01\x09\x00\x00\xf2\xf2MM\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00*M\xff\xff\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00MMMM\xff\xffM\xff\x00;MM0*\x00\x00\x00\x00\x00\x00\x0a";
    let mut stream = inflate::InflateStream::new();
    let res = stream.update(data);
    assert!(res.is_err());
}

#[test]
// no checksum present at the end of the data stream (cargo-fuzz test-case)
fn no_checksum() {
    let data = b"\x13\xff\xed\xff\xff\x12\xbfM\x00\x00\x00\x00\xd1";
    let mut stream = inflate::InflateStream::new();
    let res = stream.update(data);
    // This is not an error, because the checksum may be included in the next
    // call to `stream.update`. See issue #27.
    assert!(res.is_ok());
}

#[test]
/// The first byte of the CRC is already read into the BitStream buffer.
fn issue_26() {
    let data = &[120, 156, 189, 138, 65, 13, 0, 32, 16, 195, 64, 2, 22, 176, 128, 5, 44, 96, 1, 11,
        216, 103, 19, 176, 123, 118, 73, 155, 61, 218, 155, 54, 10, 136, 192, 170, 32, 130, 41,
        249, 36, 136, 96, 73, 62, 9, 34, 216, 146, 79, 130, 8, 142, 228, 147, 32, 130, 43, 249, 36,
        136, 224, 73, 62, 9, 32, 248, 250, 192, 22, 113, 123];
    let mut stream = inflate::InflateStream::from_zlib();
    let res = stream.update(data);
    assert!(res.is_ok());
}
