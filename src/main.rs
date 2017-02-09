use std::mem;
use std::time::Duration;
use std::net::UdpSocket;

fn encode_u16(vec: &mut Vec<u8>, num_lc: u16) {
    let num_be: u16 = num_lc.to_be();
    let num_bytes: [u8; 2] = unsafe {
        mem::transmute_copy(&num_be)
    };
    vec.extend_from_slice(&num_bytes);
}

#[test]
fn encode_u16_test() {
    let mut buffer = Vec::<u8>::new();
    encode_u16(&mut buffer, 0x1122);
    assert!(buffer[0] == 0x11, "encode_u16() ERR");
    assert!(buffer[1] == 0x22, "encode_u16() ERR");
}

const NAME_LEN : usize = 16;

fn encode_name(vec: &mut Vec<u8>, text: &str) {
    let upper_text = text.to_uppercase();
    let upper_bytes = upper_text.as_bytes();
    let mut short_bytes: [u8; 16] = [0x20; 16];
    for idx in 0..(NAME_LEN) {
        if idx < text.len() {
            short_bytes[idx] = upper_bytes[idx];
        }
    }
    let mut enc_bytes: [u8; 2*NAME_LEN+2] = [0; 2*NAME_LEN+2];
    enc_bytes[0] = 0x20;
    for idx in 0..NAME_LEN {
        enc_bytes[idx*2+1] = ((short_bytes[idx] & 0xF0) >> 4) + 0x41;
        enc_bytes[idx*2+2] = ((short_bytes[idx] & 0x0F) >> 0) + 0x41;
    }
    vec.extend_from_slice(&enc_bytes);
}

#[test]
fn encode_name_test() {
    {
        let mut buffer = Vec::<u8>::new();
        encode_name(&mut buffer, "0123456");
        let string = String::from_utf8(buffer).unwrap();
        let string_want = "\x20DADBDCDDDEDFDGCACACACACACACACACA\0";
    }
    {
        let mut buffer = Vec::<u8>::new();
        encode_name(&mut buffer, "0123456789abcdefghijklmn");
        let string = String::from_utf8(buffer).unwrap();
        let string_want = "\x20DADBDCDDDEDFDGDHDIDJEBECEDEEEFEG\0";
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

fn decode_u16(bytes: &[u8; 2]) -> u16 {
    let num_be: u16 = unsafe {
        mem::transmute_copy(bytes)
    };
    let num_lc = u16::from_be(num_be);
    return num_lc;
}

#[test]
fn decode_u16_test() {
    let bytes: [u8; 2] = [0x11, 0x22];
    let num = decode_u16(&bytes);
    assert_eq!(num, 0x1122);
}


fn decode_name(bytes: &[u8]) -> Option<String> {
    if bytes.len() < NAME_LEN*2+2 {
        return None;
    }
    if bytes[0] != 0x20 {
        return None;
    }
    let mut name_bytes: [u8; 16] = [0; 16];
    for idx in 0..NAME_LEN {
        name_bytes[idx] |= ((bytes[2*idx+1] - 0x41) & 0xF) << 4;
        name_bytes[idx] |= ((bytes[2*idx+2] - 0x41) & 0xF) << 0;
    }
    let name_end = name_bytes.iter()
        .position(|&val| { val == 0x20 || val == 0x00 })
        .unwrap();
    let name_vec: Vec<u8> = name_bytes[0..name_end].to_vec();
    let name = String::from_utf8(name_vec).unwrap();
    return Some(name);
}

#[test]
fn decode_name_test() {
    {
        let bytes: &[u8] = "\x20DADBDCDDDEDFDGCACACACACACACACACA\0".as_bytes();
        let text: String = decode_name(bytes).unwrap();
        print!("\n{}\n", text);
        assert_eq!(&text, "0123456")
    }
    {
        let bytes: &[u8] = "\x20DADBDCDDDEDFDGDHDIDJEBECEDEEEFEG\0".as_bytes();
        let text: String = decode_name(bytes).unwrap();
        print!("\n{}\n", text);
        assert_eq!(&text, "0123456789ABCDEF")
    }
}

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

    //let mut udp = UdpSocket::bind("127.0.0.1:23333").unwrap();
    //let read_timeout = Duration::from_secs(5);
    //udp.set_read_timeout(read_timeout);
    //let write_timeout = Duration::from_secs(5);
    //udp.set_write_timeout(write_timeout);
    //udp.send_to("192.168.1.10:137");
}
