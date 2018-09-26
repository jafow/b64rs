use std::string::FromUtf8Error;
use table;

#[derive(Debug)]
struct BitMask {
    be2: u8,
    be4: u8,
    be6: u8,
    le2: u8,
    le4: u8,
    le6: u8
}

const BITMASK: BitMask = BitMask {
    be2: 0xc0,
    be4: 0xf0,
    be6: 0xfc,
    le2: 0x03,
    le4: 0xf,
    le6: 0x3f
};

const SENT: u8 = 255;
const PAD: u8 = 64;

pub fn encode (s: &[u8]) -> Result<String, FromUtf8Error> {
    let len = s.len();
    let mut encoded: Vec<u8> = Vec::new();
    let mut input_str: Vec<u8> = Vec::from(s);

    // amount of "=" padding to add if number of bits doesn't divide evenly by 3 
    let mut padding: usize = (len * 8) % 3;

    while padding > 0 {
        // add byte(s) on end to round out to even length
        padding -= 1;
        input_str.push(SENT);
    }

    let mut i = 0;
    let l = input_str.len();

    while i < l - 1 {
        // loop through 3-byte chunks and mask or add padding
        let chunk = [&input_str[i as usize], &input_str[i + 1 as usize], &input_str[i + 2 as usize]];
        encoded.push((chunk[0] & BITMASK.be6) >> 2);

        if chunk[1] == &SENT && i == len - 1 {
            // reached sentinel, add padding
            encoded.push((chunk[0] & BITMASK.le2 << 4) + 16);
            encoded.push(PAD);
            encoded.push(PAD);
            break;
        } else {
            encoded.push(((chunk[0] & BITMASK.le2) << 4) + ((chunk[1] & BITMASK.be4) >> 4));

            if chunk[2] == &SENT {
                encoded.push((chunk[1] & BITMASK.le4) << 2);
                encoded.push(PAD);
                break;
            } else {
                encoded.push(((chunk[1] & BITMASK.le4) << 2) + ((chunk[2] & BITMASK.be2) >> 6));
                encoded.push(chunk[2] & BITMASK.le6);
                i += 3;
            }
        }
    }
    String::from_utf8(encoded.iter()
                      .map(|x| table::standard()[*x as usize])
                      .collect())
}
