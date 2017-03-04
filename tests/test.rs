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
// See https://github.com/PistonDevelopers/inflate/issues/14
fn issue_14() {
    let test_data = get_test_file_data("tests/issue_14.zlib");
    let res = inflate::inflate_bytes_zlib(&test_data);
    // This should fail as the file specifies code lengths that won't work.
    assert!(res.is_err());
}
