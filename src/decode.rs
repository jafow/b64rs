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
    le56: 0x30,
    le6: 0x3f
};

const SENT: u8 = 255;
const PAD: u8 = 64;

/// lookup each char in reverse table
/// ie [b"T", b"W", b"F", b"u"] => [19, 22, 5, 46] => [77, 67, 110]
/// then smoosh those ints into 3 bytes
///
/// example:
///     A = [0 0(0 1 0 0 1 1, 0 0 0 1) (0 1 1 0, 0 0 0 0 0 1) (0 1, 1 0 1 0 1 1 1 0)]
///
/// bit1 =  |0 0 0 1 0 0 1 1| << 2
///         |0 1 0 0 1 1 0 0|
///
///      +  
///
/// bit2    |0 0 0 1 0 1 0 1| & | 0 0 0 1 0 0 0 0 | 
///         |0 0 0 1 0 0 0 0|
///                  > > > >
///         |0 0 0 0 0 0 0 1|
///
///      ==
///
///         |0 1 0 0 1 1 0 1|
///
///
///
pub fn decode(s: &[u8]) -> &[u8] {
    let len = s.len();
    let mut output = Vec::new();

    for c in s.chunks(4) {
        output.push((&c[0] << 2) + (&c[1] & BITMASK.le56 >> 4));
        output.push(((&c[1] & BITMASK.be4) << 2) + ((&c[2] & BITMASK.le2) >> 2));
    }
    output
}
