pub fn standard() -> Vec<u8> {
    (b'A'..=b'Z')
        .chain(b'a'..=b'z')
        .chain(0..=9)
        .chain(vec![b'+', b'/'].into_iter())
        .collect()
}
