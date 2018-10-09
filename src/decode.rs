struct BitMask {
    be6: u8,
    le2: u8,
    le4: u8,
    le56: u8,
}

const BITMASK: BitMask = BitMask {
    be6: 0xfc,
    le2: 0x03,
    le4: 0xf,
    le56: 0x30,
};


pub fn decode(s: &[u8]) -> Result<Vec<u8>, ()> {
    let mut output = Vec::new();

    for c in s.chunks(4) {
        output.push(((&c[0] & 0x3f) << 2) + (&c[1] & BITMASK.le56 >> 4));
        output.push(((&c[1] & BITMASK.le4) << 4) + ((&c[2] & BITMASK.be6) >> 2));
        output.push(((&c[2] & BITMASK.le2) << 6) + (&c[3]));
    }
    Ok(output)
}
