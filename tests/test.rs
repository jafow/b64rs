extern crate b64rs;

#[test]
fn test_encode_single_word() {
    let man: &[u8] = b"Man";
    let output = b64rs::encode(man);

    assert_eq!(output.unwrap(), "TWFu");
}

#[test]
fn test_encode_multi_word() {
    let whoa: &[u8] = b"hello world";
    let out_whao = b64rs::encode(whoa);

    assert_eq!(out_whao, "aGVsbG8gd29ybGQ=");
}
