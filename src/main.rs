use std::mem::transmute;
use std::time::Duration;
use std::net::UdpSocket;

fn encode_u16(vec: &mut Vec<u8>, num_lc: u16) {
    let num_be: u16 = num_lc.to_be();
    vec.push(((num_be & 0xFF00) >> 8) as u8);
    vec.push(((num_be & 0x00FF) >> 0) as u8);
}

fn encode_u16_test() {
    let mut buffer = Vec::<u8>::new();
    encode_u16(&mut buffer, 0x0102);
    assert!(buffer[0] == 0x02, "encode_u16() ERR");
    assert!(buffer[1] == 0x01, "encode_u16() ERR");
    println!("encode_u16() OK");
}

const NAME_LEN : usize = 16;

fn encode_name(vec: &mut Vec<u8>, text: &str) {
    let upper_text = text.to_uppercase();
    let upper_bytes = upper_text.as_bytes();
    let mut short_bytes: [u8; 16] = [32; 16];
    for idx in 0..(NAME_LEN) {
        if idx < text.len() {
            short_bytes[idx] = upper_bytes[idx];
        }
    }
    let mut enc_bytes: [u8; 33] = [0; 33];
    enc_bytes[0] = 0x20;
    for idx in 0..NAME_LEN {
        enc_bytes[idx*2+1] = ((short_bytes[idx] & 0xF0) >> 4) + 0x41;
        enc_bytes[idx*2+2] = ((short_bytes[idx] & 0x0F) >> 0) + 0x41;
    }
    vec.extend_from_slice(&enc_bytes);
}

fn encode_name_test() {
    {
        let mut buffer = Vec::<u8>::new();
        encode_name(&mut buffer, "0123456");
        let string = String::from_utf8(buffer).unwrap();
        let string_want = "\x20DADBDCDDDEDFDGCACACACACACACACACA";
        assert!(string == string_want, "encode_name() ERR");
    }
    {
        let mut buffer = Vec::<u8>::new();
        encode_name(&mut buffer, "0123456789abcdefghijklmn");
        let string = String::from_utf8(buffer).unwrap();
        let string_want = "\x20DADBDCDDDEDFDGDHDIDJEBECEDEEEFEG";
        assert!(string == string_want, "encode_name() ERR");
    }
    println!("encode_name() OK");
}

struct NbstatRequest {
    transaction_id: u16,
    flags: u16,
    questions: u16,
    answers: u16,
    authority: u16,
    extra: u16,
    user_name: String,
    query_type: u16,
    query_class: u16,
}

impl NbstatRequest {
    fn encode(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        encode_u16(&mut buffer, self.transaction_id);
        encode_u16(&mut buffer, self.flags);
        encode_u16(&mut buffer, self.questions);
        encode_u16(&mut buffer, self.answers);
        encode_u16(&mut buffer, self.authority);
        encode_u16(&mut buffer, self.extra);
        encode_name(&mut buffer, &self.user_name);
        encode_u16(&mut buffer, self.query_type);
        encode_u16(&mut buffer, self.query_class);
        return buffer;
    }
}

fn decode_u16(vec: &Vec<u8>, pos: usize) -> u16 {
    let mut num_be: u16 = 0;
    num_be |= (vec[pos+0] << 8) as u16;
    num_be |= (vec[pos+1] << 0) as u16;
    let num_lc = u16::from_be(num_be);
    return num_lc;
}
/*
fn decode_name(slice: &[u8], pos: usize) -> String {
    let mut name_bytes: [u8; 16] = [0; 16];
    for idx in 0..NAME_LEN {
        name_bytes[idx] |= ((slice[pos+2*idx+1] & 0xF0) >> 4) - 0x41;
        name_bytes[idx] |= ((slice[pos+2*idx+2] & 0x0F) >> 0) - 0x41;
    }
    let name = unsafe {
        String::from_utf8_lossy(&name_bytes)
    };
    return name;
}*/

fn main() {
    let request = NbstatRequest{
        transaction_id: 0x1337,
        flags: 0x0000,
        questions: 1,
        answers: 0,
        authority: 0,
        extra: 0,
        user_name: "123456".to_string(),
        query_type: 0x0021,
        query_class: 0x0001,
    };
    let reqBuffer = request.encode();

    encode_u16_test();
    encode_name_test();
    //let mut udp = UdpSocket::bind("127.0.0.1:23333").unwrap();
    //let read_timeout = Duration::from_secs(5);
    //udp.set_read_timeout(read_timeout);
    //let write_timeout = Duration::from_secs(5);
    //udp.set_write_timeout(write_timeout);
    //udp.send_to("192.168.1.10:137");
}
