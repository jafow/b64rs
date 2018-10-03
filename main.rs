// extern crate b64;
// use b64::*;

// fn main () {
//     let man: &str = "M";
//     let output = b64::b64::encode(man);

//     assert_eq!(output, "TQ==");

//     let whoa: &str = "hello world";
//     let out_whao = b64::b64::encode(whoa);

//     assert_eq!(out_whao, "aGVsbG8gd29ybGQ=");

//     #[cfg(test)]
//     mod tests {
//         #[test]
//         fn test_encode() {
//             let hex_in = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
//             let b64_out = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
//             assert_eq!(b64_out, b64::b64::encode(&hex_in));
//         }
//     }
// }
