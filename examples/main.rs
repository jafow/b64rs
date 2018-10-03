extern crate b64rs;

fn main() {
    let man = b"TWFu";
    let res = b64rs::decode(man);
    let s = ['l', 'o', 'r', 'e', 'm'];

    println!("s at 1 = 0", &s[1]);

    // assert_eq!(vec![8,2,3, 4, 1, 2, 3], res);
}
