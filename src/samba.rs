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
    mac_key: String
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
            mac_key: String::from(name)
        };
    }
}

const VEC_INIT_SIZE: usize = 64;

const SMB_COM_NEGOTIATE: u8 = 0x72;

#[allow(dead_code)]
fn encode_header(context: &Context, command: u8) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(VEC_INIT_SIZE);
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

#[allow(dead_code)]
fn encode_packet(context: &Context, header: &[u8], param: &[u8], data: &[u8]) -> Vec<u8> {
    let mut packet: Vec<u8> = Vec::with_capacity(VEC_INIT_SIZE);
    packet.extend_from_slice(&[0, 0, 0, 0]);
    // header
    packet.extend_from_slice(header);
    // parameter
    let param_len: u8 = (param.len() / 2) as u8;
    packet.extend_from_slice(&encode_u8_le(param_len));
    packet.extend_from_slice(param);
    // data
    let data_len: u16 = data.len() as u16;
    packet.extend_from_slice(&encode_u16_le(data_len));
    packet.extend_from_slice(data);
    // size
    let packet_size: u32 = (packet.len() - 4) as u32;
    packet[0..4].copy_from_slice(&encode_u32_be(packet_size));
    return packet;
}

#[allow(dead_code)]
fn encode_negotiate_protocol(context: &Context) -> Vec<u8> {
    // header
    let header: Vec<u8> = encode_header(&context, SMB_COM_NEGOTIATE);
    // param
    let mut param: Vec<u8> = vec![0; 0];
    // data
    let mut data: Vec<u8> = Vec::with_capacity(VEC_INIT_SIZE);
    data.extend_from_slice(&encode_u8_le(2));
    data.extend_from_slice("NT LM 0.12\0".as_bytes());
    data.extend_from_slice(&encode_u8_le(2));
    data.extend_from_slice("\0".as_bytes());
    // packet
    let packet = encode_packet(&context, &header, &param, &data);
    return packet;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::fs::File;

    #[test]
    fn encode_header_test() {
        let context: Context = Context::new("192.168.1.1:445", "");
        let buffer: Vec<u8> = encode_header(&context, SMB_COM_NEGOTIATE);
        let mut file: File = File::open("./dump/smb_encode_header.dump").unwrap();
        let mut fileBuf: Vec<u8> = Vec::new();
        file.read_to_end(&mut fileBuf);
        assert_eq!(buffer, fileBuf);
    }

    #[test]
    fn encode_negotiate_protocol_test() {
        let context: Context = Context::new("192.168.1.1:445", "");
        let buffer: Vec<u8> = encode_negotiate_protocol(&context);
        let mut file: File = File::open("./dump/smb_encode_negotiate_protocol.dump").unwrap();
        let mut fileBuf: Vec<u8> = Vec::new();
        file.read_to_end(&mut fileBuf);
        assert_eq!(buffer, fileBuf);
    }
}
