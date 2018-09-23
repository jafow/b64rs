pub mod b64 {
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

    const TABLE: [&str; 65] = ["A", "B", "C", "D", "E", "F", "G", "H", "I",
    "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X",
    "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i",
    "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x",
    "y", "z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "+", "/", "="];

    pub fn encode (s: &str) -> String {
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

        encoded.iter().map(|x| TABLE[*x as usize]).collect()
    }
}
