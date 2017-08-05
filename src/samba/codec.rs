use std::mem;

#[allow(dead_code)]
pub fn encode_u8_be(num_lc: u8) -> [u8; 1] {
    let num_bytes: [u8; 1] = [num_lc];
    return num_bytes;
}

#[allow(dead_code)]
pub fn encode_u16_be(num_lc: u16) -> [u8; 2] {
    let num_be: u16 = num_lc.to_be();
    let num_bytes: [u8; 2] = unsafe {
        mem::transmute_copy(&num_be)
    };
    return num_bytes;
}

#[allow(dead_code)]
pub fn encode_u32_be(num_lc: u32) -> [u8; 4] {
    let num_be: u32 = num_lc.to_be();
    let num_bytes: [u8; 4] = unsafe {
        mem::transmute_copy(&num_be)
    };
    return num_bytes;
}

#[allow(dead_code)]
pub fn encode_u64_be(num_lc: u64) -> [u8; 8] {
    let num_be: u64 = num_lc.to_be();
    let num_bytes: [u8; 8] = unsafe {
        mem::transmute_copy(&num_be)
    };
    return num_bytes;
}

#[allow(dead_code)]
pub fn decode_u8_be(bytes: &[u8]) -> u8 {
    return bytes[0]
}

#[allow(dead_code)]
pub fn decode_u16_be(bytes: &[u8]) -> u16 {
    let tmp_bytes: [u8; 2] = [
        bytes[0], bytes[1]
    ];
    let num_be: u16 = unsafe {
        mem::transmute_copy(&tmp_bytes)
    };
    let num_lc: u16 = u16::from_be(num_be);
    return num_lc;
}

#[allow(dead_code)]
pub fn decode_u32_be(bytes: &[u8]) -> u32 {
    let tmp_bytes: [u8; 4] = [
        bytes[0], bytes[1],
        bytes[2], bytes[3]
    ];
    let num_be: u32 = unsafe {
        mem::transmute_copy(&tmp_bytes)
    };
    let num_lc: u32 = u32::from_be(num_be);
    return num_lc;
}

#[allow(dead_code)]
pub fn decode_u64_be(bytes: &[u8]) -> u64 {
    let tmp_bytes: [u8; 8] = [
        bytes[0], bytes[1],
        bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7]
    ];
    let num_be: u64 = unsafe {
        mem::transmute_copy(&tmp_bytes)
    };
    let num_lc: u64 = u64::from_be(num_be);
    return num_lc;
}

#[allow(dead_code)]
pub fn encode_u8_le(num_lc: u8) -> [u8; 1] {
    let num_bytes: [u8; 1] = [num_lc];
    return num_bytes;
}

#[allow(dead_code)]
pub fn encode_u16_le(num_lc: u16) -> [u8; 2] {
    let num_be: u16 = num_lc.to_le();
    let num_bytes: [u8; 2] = unsafe {
        mem::transmute_copy(&num_be)
    };
    return num_bytes;
}

#[allow(dead_code)]
pub fn encode_u32_le(num_lc: u32) -> [u8; 4] {
    let num_be: u32 = num_lc.to_le();
    let num_bytes: [u8; 4] = unsafe {
        mem::transmute_copy(&num_be)
    };
    return num_bytes;
}

#[allow(dead_code)]
pub fn encode_u64_le(num_lc: u64) -> [u8; 8] {
    let num_be: u64 = num_lc.to_le();
    let num_bytes: [u8; 8] = unsafe {
        mem::transmute_copy(&num_be)
    };
    return num_bytes;
}

#[allow(dead_code)]
pub fn decode_u8_le(bytes: &[u8]) -> u8 {
    return bytes[0]
}

#[allow(dead_code)]
pub fn decode_u16_le(bytes: &[u8]) -> u16 {
    let tmp_bytes: [u8; 2] = [
        bytes[1], bytes[0]
    ];
    let num_be: u16 = unsafe {
        mem::transmute_copy(&tmp_bytes)
    };
    let num_lc: u16 = u16::from_be(num_be);
    return num_lc;
}

#[allow(dead_code)]
pub fn decode_u32_le(bytes: &[u8]) -> u32 {
    let tmp_bytes: [u8; 4] = [
        bytes[3], bytes[2],
        bytes[1], bytes[0]
    ];
    let num_be: u32 = unsafe {
        mem::transmute_copy(&tmp_bytes)
    };
    let num_lc: u32 = u32::from_be(num_be);
    return num_lc;
}

#[allow(dead_code)]
pub fn decode_u64_le(bytes: &[u8]) -> u64 {
    let tmp_bytes: [u8; 8] = [
        bytes[7], bytes[6],
        bytes[5], bytes[4],
        bytes[3], bytes[2],
        bytes[1], bytes[0]
    ];
    let num_be: u64 = unsafe {
        mem::transmute_copy(&tmp_bytes)
    };
    let num_lc: u64 = u64::from_be(num_be);
    return num_lc;
}

#[cfg(test)]
mod tests {
    use codec::*;

    #[test]
    fn encode_u8_be_test() {
        let bytes = encode_u8_be(0x11);
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x11);
    }

    #[test]
    fn encode_u16_be_test() {
        let bytes = encode_u16_be(0x1122);
        assert_eq!(bytes.len(), 2);
        assert_eq!(bytes[0], 0x11);
        assert_eq!(bytes[1], 0x22);
    }

    #[test]
    fn encode_u32_be_test() {
        let bytes = encode_u32_be(0x11223344);
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes[0], 0x11);
        assert_eq!(bytes[1], 0x22);
        assert_eq!(bytes[2], 0x33);
        assert_eq!(bytes[3], 0x44);
    }

    #[test]
    fn encode_u64_be_test() {
        let bytes = encode_u64_be(0x1122334455667788);
        assert_eq!(bytes.len(), 8);
        assert_eq!(bytes[0], 0x11);
        assert_eq!(bytes[1], 0x22);
        assert_eq!(bytes[2], 0x33);
        assert_eq!(bytes[3], 0x44);
        assert_eq!(bytes[4], 0x55);
        assert_eq!(bytes[5], 0x66);
        assert_eq!(bytes[6], 0x77);
        assert_eq!(bytes[7], 0x88);
    }

    #[test]
    fn decode_u8_be_test() {
        let bytes: &[u8] = &[0x11];
        let num: u8 = decode_u8_be(bytes);
        assert_eq!(num, 0x11);
    }

    #[test]
    fn decode_u16_be_test() {
        let bytes: &[u8] = &[0x11, 0x22];
        let num: u16 = decode_u16_be(bytes);
        assert_eq!(num, 0x1122);
    }

    #[test]
    fn decode_u32_be_test() {
        let bytes: &[u8] = &[0x11, 0x22, 0x33, 0x44];
        let num: u32 = decode_u32_be(bytes);
        assert_eq!(num, 0x11223344);
    }

    #[test]
    fn decode_u64_be_test() {
        let bytes: &[u8] = &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
        let num: u64 = decode_u64_be(bytes);
        assert_eq!(num, 0x1122334455667788);
    }

    #[test]
    fn decode_u8_le_test() {
        let bytes: &[u8] = &[0x11];
        let num: u8 = decode_u8_le(bytes);
        assert_eq!(num, 0x11);
    }

    #[test]
    fn decode_u16_le_test() {
        let bytes: &[u8] = &[0x22, 0x11];
        let num: u16 = decode_u16_le(bytes);
        assert_eq!(num, 0x1122);
    }

    #[test]
    fn decode_u32_le_test() {
        let bytes: &[u8] = &[0x44, 0x33, 0x22, 0x11];
        let num: u32 = decode_u32_le(bytes);
        assert_eq!(num, 0x11223344);
    }

    #[test]
    fn decode_u64_le_test() {
        let bytes: &[u8] = &[0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11];
        let num: u64 = decode_u64_le(bytes);
        assert_eq!(num, 0x1122334455667788);
    }

    #[test]
    fn encode_u8_le_test() {
        let bytes = encode_u8_le(0x11);
        assert_eq!(bytes.len(), 1);
        assert_eq!(bytes[0], 0x11);
    }

    #[test]
    fn encode_u16_le_test() {
        let bytes = encode_u16_le(0x1122);
        assert_eq!(bytes.len(), 2);
        assert_eq!(bytes[0], 0x22);
        assert_eq!(bytes[1], 0x11);
    }

    #[test]
    fn encode_u32_le_test() {
        let bytes = encode_u32_le(0x11223344);
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes[0], 0x44);
        assert_eq!(bytes[1], 0x33);
        assert_eq!(bytes[2], 0x22);
        assert_eq!(bytes[3], 0x11);
    }

    #[test]
    fn encode_u64_le_test() {
        let bytes = encode_u64_le(0x1122334455667788);
        assert_eq!(bytes.len(), 8);
        assert_eq!(bytes[0], 0x88);
        assert_eq!(bytes[1], 0x77);
        assert_eq!(bytes[2], 0x66);
        assert_eq!(bytes[3], 0x55);
        assert_eq!(bytes[4], 0x44);
        assert_eq!(bytes[5], 0x33);
        assert_eq!(bytes[6], 0x22);
        assert_eq!(bytes[7], 0x11);
    }
}
