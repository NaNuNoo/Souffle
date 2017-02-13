use codec::*;

#[allow(dead_code)]
struct Context {
    uid: u16,
    tid: u16,
    mid: u16,
    pid: u16,
    sequence: i32,
    host: String,
    name: String,
}

#[allow(dead_code)]
impl Context {
    fn new(host: &str, name: &str) -> Context {
        return Context{
            uid: 0,
            tid: 0,
            mid: 1,
            pid: 12345,
            sequence: -1,
            host: String::from(host),
            name: String::from(name),
        };
    }
}

const HEADER_SIZE: usize = 32;
const NEGOTIATE_PROTOCOL_SIZE: usize = 14;

const SMB_COM_NEGOTIATE: u8 = 0x72;

#[allow(dead_code)]
fn encode_header(context: &Context, command: u8) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(HEADER_SIZE);
    buffer.extend_from_slice(&[
        0xFF, ('S' as u8), ('M' as u8), ('B' as u8)
    ]);
    buffer.extend_from_slice(&encode_u8_le(command));
    buffer.extend_from_slice(&encode_u32_le(0));
    buffer.extend_from_slice(&encode_u8_le(0x18));
    buffer.extend_from_slice(&encode_u16_le(0x6045));
    buffer.extend_from_slice(&encode_u16_le(0));
    buffer.extend_from_slice(&encode_u64_le(0));
    buffer.extend_from_slice(&encode_u16_le(0));
    buffer.extend_from_slice(&encode_u16_le(context.tid));
    buffer.extend_from_slice(&encode_u16_le(context.pid));
    buffer.extend_from_slice(&encode_u16_le(context.uid));
    buffer.extend_from_slice(&encode_u16_le(context.mid));
    return buffer;
}

#[derive(Default)]
struct NegotiateProtocol {
}

#[allow(dead_code)]
fn encode_negotiate_protocol(args: &NegotiateProtocol) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(NEGOTIATE_PROTOCOL_SIZE);
    buffer.extend_from_slice(&encode_u8_le(2));
    buffer.extend_from_slice("NT LM 0.12\0".as_bytes());
    buffer.extend_from_slice(&encode_u8_le(2));
    buffer.extend_from_slice("\0".as_bytes());
    return buffer;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::fs::File;

    #[test]
    fn encode_header_test() {
        let context: Context = Context::new("192.168.1.8:445", "");
        let buffer: Vec<u8> = encode_header(&context, SMB_COM_NEGOTIATE);
        let mut file: File = File::open("./dump/smb_encode_header.dump").unwrap();
        let mut fileBuf: Vec<u8> = Vec::new();
        file.read_to_end(&mut fileBuf);
        assert_eq!(buffer, fileBuf);
    }
}
