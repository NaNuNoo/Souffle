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

struct Header {
    samba: [u8; 4],
    command: u8,
    status: u32,
    flags: u8,
    flags2: u16,
    pid_high: u8,
    signature: u64,
    unused: u16,
    tid: u16,
    uid: u16,
    pid: u16,
    mid: u16,
}

impl Header {
    fn new_empty() -> {
        return Header {
            samba: [0; 4],
            command: 0,
            status: 0,
            flags: 0,
            flags2: 0,
            pid_high: 0,
            signature: 0,
            unused: 0,
            tid: 0,
            uid: 0,
            pid: 0,
            mid: 0,
        }
    }

    fn new_default() -> {
        return Header {
            samba: SMB_REQUEST_SIGN,
            command: 0,
            status: 0,
            flags: 0x18,
            flags2: 0x6045,
            pid_high: 0,
            signature: 0,
            unused: 0,
            tid: 0,
            uid: 0,
            pid: 0,
            mid: 0,
        }
    }


    #[allow(dead_code)]
    fn encode(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(VEC_INIT_SIZE);
        buffer.extend_from_slice(&self.samba);
        buffer.extend_from_slice(&encode_u8_le(self.command));
        buffer.extend_from_slice(&encode_u32_le(self.status));
        buffer.extend_from_slice(&encode_u8_le(self.flags));
        buffer.extend_from_slice(&encode_u16_le(self.flags2));
        buffer.extend_from_slice(&encode_u16_le(self.pid_high));
        buffer.extend_from_slice(&encode_u64_le(self.signature));
        buffer.extend_from_slice(&encode_u16_le(self.unused));
        buffer.extend_from_slice(&encode_u16_le(self.tid));
        buffer.extend_from_slice(&encode_u16_le(self.pid));
        buffer.extend_from_slice(&encode_u16_le(self.uid));
        buffer.extend_from_slice(&encode_u16_le(self.mid));
        return buffer;
    }

    #[allow(dead_code)]
    fn decode(&self, buffer: &[u8]) {
        self.sign = header[0..4];
        self.command = decode_u8_le(header[4..5]);
        self.status = decode_u8_le(header[5..9]);
        self.flags = decode_u8_le(header[9..10]);
        self.flags2 = decode_u8_le(header[10..12]);
        self.pid_high = decode_u16_le(header[12..14]);
        self.signature = decode_u64_le(header[14..22]);
        self.unused = decode_u16_le(header[22..24]);
        self.tid = decode_u16_le(header[24..26]);
        self.pid = decode_u16_le(header[26..28]);
        self.uid = decode_u16_le(header[28..30]);
        self.mid = decode_u16_le(header[30..32]);
    }
}

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

    #[allow(dead_code)]
    fn encode_negotiate(&mut self) -> Vec<u8> {
        // header
        let mut header = Header::new_default();
        header.command = SMB_COM_NEGOTIATE;
        header.mid = self.mid;
        header.pid = self.pid;
        // parameter
        let mut param: Vec<u8> = vec![0; 0];
        // data
        let mut data: Vec<u8> = Vec::with_capacity(VEC_INIT_SIZE);
        data.extend_from_slice(&encode_u8_le(2));
        data.extend_from_slice("NT LM 0.12\0".as_bytes());
        data.extend_from_slice(&encode_u8_le(2));
        data.extend_from_slice("\0".as_bytes());
        // packet
        let packet = _encode_packet(&self, &header, &param, &data);
        return packet;
    }

    fn decode_negotiate(&mut self, buffer: &Vec<u8>) -> bool {
        // header
        let mut header = Header::new_empty();
        header.decode(buffer[0..SMB_HEADER_SIZE]);
        if header.samba != SMB_RESPONSE_SIGN {
            return false;
        }
        self.extended_security = !(header.flags2 & 0x0800);
        // parameter
        self.security_mode = decode_u8_le(param_buf[0..1]);
        self.max_mpx = decode_u16_le(param_buf[1..3]);
        self.max_vc = decode_u16_le(param_buf[3..5]);
        self.max_buffer = decode_u32_le(param_buf[5..9]);
        self.max_raw_buffer = decode_u32_le(param_buf[9..13]);
        self.session_key = decode_u32_le(param_buf[13..17]);
        self.capabilities = decode_u32_le(param_buf[17..21]);
        if (param_buf.len() >= 25) {
            self.time = decode_u64_le(param_buf[21..25]);
        }
        if (param_buf.len() >= 27) {
            self.timezone = decode_u16_le(param_buf[25..27]);
        }
        if (param_buf.len() >= 28) {
            self.key_length = decode_u8_le(param_buf[27..28]);
        }
        // data
        if self.extended_security {
            let offset: usize = 0;
            self.server_challenge = data[0..self.key_length];
            offset = offset + self.key_length;
            self.server_guid = data[offset..offset+16];
            offset = offset + 16;
            seld.security_blob = data[offset..];
        }
    }
}

#[allow(dead_code)]
fn decode_negotiate_protocol(self: &Context, header_buf: &Vec<u8>) -> Option<> {
    let header: &[u8] = header_buf[0..4];
    if
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
