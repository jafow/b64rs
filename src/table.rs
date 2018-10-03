use std::collections::HashMap;
pub fn standard() -> Vec<u8> {
    (b'A'..=b'Z')
        .chain(b'a'..=b'z')
        .chain(0..=9)
        .chain(vec![b'+', b'/'].into_iter())
        .collect()
}

pub fn decode () {
    let mut h: HashMap<&u8, &u8> = HashMap::new();
    let t = standard().iter();
    for (k, v) in t.zip(0..=64) {
        h.insert(k, &v);
    }
    println!("h is {:?}", h);
}
