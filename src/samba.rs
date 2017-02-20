use codec::*;

const VEC_INIT_SIZE: usize = 64;

const SMB_REQUEST_SIGN: [u8; 4] = [0xFF, ('S' as u8), ('M' as u8), ('B' as u8)];
const SMB_RESPONSE_SIGN: [u8; 4] = [0xFE, ('S' as u8), ('M' as u8), ('B' as u8)];
const SMB_HEADER_SIZE: u8 = 32;

const SMB_COM_NEGOTIATE: u8 = 0x72;
const SMB_COM_SESSION_SETUP_ANDX: u8 = 0x73;

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

//
// header
//

#[derive(Default)]
struct EncodeHeader {
    command: u8,
    pid: u16,
    mid: u16,
}

#[allow(dead_code)]
fn encode_header(header: &EncodeHeader) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(VEC_INIT_SIZE);
    buffer.extend_from_slice(&SMB_REQUEST_SIGN);
    buffer.extend_from_slice(&encode_u8_le(header.command));
    buffer.extend_from_slice(&encode_u32_le(0));
    buffer.extend_from_slice(&encode_u8_le(0x18));
    buffer.extend_from_slice(&encode_u16_le(0x6045));
    buffer.extend_from_slice(&encode_u16_le(0));
    buffer.extend_from_slice(&encode_u64_le(0));
    buffer.extend_from_slice(&encode_u16_le(0));
    buffer.extend_from_slice(&encode_u16_le(0));
    buffer.extend_from_slice(&encode_u16_le(header.pid));
    buffer.extend_from_slice(&encode_u16_le(0));
    buffer.extend_from_slice(&encode_u16_le(header.mid));
    return buffer;
}

#[derive(Default)]
struct DecodeHeader {
    command: u8,
    flags2: u16,
    signature: u64,
}

#[allow(dead_code)]
fn decode_header(buffer: &[u8]) -> Option<DecodeHeader> {
    let header = DecodeHeader::default();
    if buffer.len() != SMB_HEADER_SIZE {
        return None;
    }
    let sign: &[u8] = header[0..4];
    if sign != SMB_RESPONSE_SIGN {
        return None;
    }
    header.command = decode_u8_le(header[4..5]);
    //header.status = decode_u8_le(header[5..9]);
    //header.flags = decode_u8_le(header[9..10]);
    header.flags2 = decode_u8_le(header[10..12]);
    //header.pid_high = decode_u16_le(header[12..14]);
    header.signature = decode_u64_le(header[14..22]);
    //header.unused = decode_u16_le(header[22..24]);
    //header.tid = decode_u16_le(header[24..26]);
    //header.pid = decode_u16_le(header[26..28]);
    //header.uid = decode_u16_le(header[28..30]);
    //header.mid = decode_u16_le(header[30..32]);
    return Some(header);
}

//
// negotiate param
//

#[derive(Default)]
struct EncodeNegotiateParam {
}

fn encode_negotiate_param(param: &EncodeNegotiateParam) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(VEC_INIT_SIZE);
    return buffer;
}

#[derive(Default)]
struct DecodeNegotiateParam {
    security_mode: u8,
    max_mpx: u16,
    max_vc: u16,
    max_buffer: u32,
    max_raw_buffer: u32,
    session_key: u32,
    capabilities: u32,
    time: u64,
    timezone: u16,
    key_length: u8,
}

fn decode_negotiate_param(buffer: &[u8]) -> Option<DecodeNegotiateParam> {
    let param = DecodeNegotiateParam::default();
    // parameter
    param.security_mode = decode_u8_le(param_buf[0..1]);
    param.max_mpx = decode_u16_le(param_buf[1..3]);
    param.max_vc = decode_u16_le(param_buf[3..5]);
    param.max_buffer = decode_u32_le(param_buf[5..9]);
    param.max_raw_buffer = decode_u32_le(param_buf[9..13]);
    param.session_key = decode_u32_le(param_buf[13..17]);
    param.capabilities = decode_u32_le(param_buf[17..21]);
    if (param_buf.len() >= 25) {
        param.time = decode_u64_le(param_buf[21..25]);
    }
    if (param_buf.len() >= 27) {
        param.timezone = decode_u16_le(param_buf[25..27]);
    }
    if (param_buf.len() >= 28) {
        param.key_length = decode_u8_le(param_buf[27..28]);
    }
    return Some(param);
}

//
// negotiate data
//

#[derive(Default)]
struct EncodeNegotiateData {
}

fn encode_negotiate_data(param: &EncodeNegotiateData) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(VEC_INIT_SIZE);
    buffer.extend_from_slice(&encode_u8_le(2));
    buffer.extend_from_slice("NT LM 0.12\0".as_bytes());
    buffer.extend_from_slice(&encode_u8_le(2));
    buffer.extend_from_slice("\0".as_bytes());
    return buffer;
}

#[derive(Default)]
struct DecodeNegotiateData {
    server_challenge: Vec<u8>,
    server_guid: Vec<u8>,
    security_blob: Vec<u8>,
}

fn decode_negotiate_data(buffer: &[u8], key_length: usize) -> Option<DecodeNegotiateData> {
    let data = DecodeNegotiateData::default();
    let offset: usize = 0;
    data.server_challenge = data[0..key_length];
    offset = offset + self.key_length;
    data.server_guid = data[offset..offset+16];
    offset = offset + 16;
    self.security_blob = data[offset..];
    return Some(data);
}

//
// negotiate
//

#[allow(dead_code)]
fn encode_negotiate() -> Vec<u8> {
    // header
    let header = EncodeHeader::default();
    header.command = SMB_COM_NEGOTIATE;
    header.mid = 1;
    header.pid = 12345;
    let header_buf = encode_header(&header);
    // param
    let param = EncodeNegotiateParam();
    let param_buf = encode_negotiate_param(&param);
    // data
    let data = EncodeNegotiateData();
    let data_buf = encode_negotiate_data(&data);
    // packet
    let packet = encode_packet(&header_buf, &param_buf, &data_buf);
    return packet;
}

#[derive(Default)]
struct DecodeNegotiate {
    extended_security: bool,
    server_challenge: Vec<u8>,
    server_guid: Vec<u8>,
}

fn decode_negotiate(buffer: &Vec<u8>) -> Option<DecodeNegotiate> {
    let result = DecodeNegotiate::default();
    let (header_buf, param_buf, data_buf) = decode_packet(buffer);
    // header
    let header: DecodeHeader = decode_header(&header).unwrap();
    result.extended_security = !(header.flags2 & 0x0800)
    // param
    let param: DecodeNegotiateParam = decode_negotiate_param(&param_buf).unwrap();
    // data
    if result.extended_security {
        let data: DecodeNegotiateData = decode_negotiate_data(&data_buf).unwrap();
        result.server_challenge = data.server_challenge;
        result.server_guid = data.server_guid;
    }
    return result;
}

//
// 
//

#[allow(dead_code)]
impl Context {
    fn new(host: &str, name: &str) -> Context {
        return Context{
            mid: 1,
            pid: 12345,
            sequence: -1,
            host: String::from(host),
            name: String::from(name),
            mac_key: String::from(name)
        };
    }

    #[allow(dead_code)]
    fn _encode_packet(self: &Context, header: &[u8], param: &[u8], data: &[u8]) -> Vec<u8> {
        let mut packet: Vec<u8> = Vec::with_capacity(VEC_INIT_SIZE);
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
        return packet;
    }

    #[allow(dead_code)]
    fn _decode_packet(self: &Context, packet: &Vec<u8>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        // header
        let header: &[u8] = packet[0..SMB_HEADER_SIZE];
        // parameter
        let param_len: usize = decode_u8_le(packet[SMB_HEADER_SIZE..1]);
        let param_start: usize = SMB_HEADER_SIZE + 1;
        let param_finish: usize = param_start + param_len;
        let param: &[u8] = packet[param_start..param_finish];
        // data
        let data_len: usize = decode_u16_le(packet[param_finish..param_finish+2]);
        let data_start: usize = data_finish + 2;
        let data_finish: usize = data_start + data_len;
        let data: &[u8] = packet[data_start..data_finish];
        return (header, param, data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::fs::File;

    #[test]
    fn encode_header_test() {
        let self: Context = Context::new("192.168.1.1:445", "");
        let buffer: Vec<u8> = _encode_header(&self, SMB_COM_NEGOTIATE);
        let mut file: File = File::open("./dump/smb_encode_header.dump").unwrap();
        let mut fileBuf: Vec<u8> = Vec::new();
        file.read_to_end(&mut fileBuf);
        assert_eq!(buffer, fileBuf);
    }

    #[test]
    fn encode_negotiate_protocol_test() {
        let self: Context = Context::new("192.168.1.1:445", "");
        let buffer: Vec<u8> = encode_negotiate_protocol(&self);
        let mut file: File = File::open("./dump/smb_encode_negotiate_protocol.dump").unwrap();
        let mut fileBuf: Vec<u8> = Vec::new();
        file.read_to_end(&mut fileBuf);
        assert_eq!(buffer, fileBuf);
    }
}
