extern crate b64rs;

#[test]
fn test_single_char() {
    let man: &str = "M";
    let output = b64rs::encode(man);

    assert_eq!(output, "TQ==");
}

#[test]
fn test_multi_word() {
    let whoa: &str = "hello world";
    let out_whao = b64rs::encode(whoa);

    assert_eq!(out_whao, "aGVsbG8gd29ybGQ=");
}
