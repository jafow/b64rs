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

    const TABLE: [&str; 64] = ["A", "B", "C", "D", "E", "F", "G", "H", "I",
    "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X",
    "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i",
    "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x",
    "y", "z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "+", "/"];

    pub fn encode (s: &str) -> String {
        let mut encoded: Vec<u8> = Vec::new();
        let mut out_str = String::new();

        let len = s.len();
        let mut boo: Vec<u8> = Vec::from(s);

        // amount of "=" padding to add if number of bits doesn't divide evenly by 3 
        let mut padding: usize = (len * 8) % 3;

        while padding > 0 {
            padding -= 1;
            boo.push(0);
        }

        for val in (0..len - 1).step_by(3) {
            let chunk = [&boo[val as usize], &boo[val + 1 as usize], &boo[val + 2 as usize]];
            encoded.push((chunk[0] & BITMASK.be6) >> 2);
            encoded.push(((chunk[0] & BITMASK.le2) << 4) + ((chunk[1] & BITMASK.be4) >> 4));
            encoded.push(((chunk[1] & BITMASK.le4) << 2) + ((chunk[2] & BITMASK.be2) >> 6));
            encoded.push(chunk[2] & BITMASK.le6);
        }
        "Hey Man".to_string()
    }
}
