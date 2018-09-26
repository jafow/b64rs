use table;

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

pub fn decode(s: Vec<u8>) -> Vec<u8> {
    vec![0]
}
