extern crate b64rs;

fn main() {
    let res = b64rs::decode(&[19, 22, 5, 46]); // "TWFu"
    println!("res {:?}", res);

    assert_eq!(res.unwrap(), b"Man");
}
