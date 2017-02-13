use std::time;
use std::net;
use codec::*;

const NAME_LEN: usize = 16;
const ENC_NAME_LEN: usize = NAME_LEN * 2 + 2;
const REQ_FIXED_LEN: usize = 55;
const RES_FIXED_LEN: usize = 57;
const HOST_NAME_LEN: usize = 18;

fn encode_name(text: &str) -> [u8; ENC_NAME_LEN] {
    let padding: u8 =
        if text == "*" {
            '\0' as u8
        } else {
            ' ' as u8
        };
    let upper_text = text.to_uppercase();
    let upper_bytes = upper_text.as_bytes();
    let mut enc_bytes: [u8; ENC_NAME_LEN] = [0; ENC_NAME_LEN];
    enc_bytes[0] = 0x20;
    enc_bytes[ENC_NAME_LEN-1] = 0;
    for idx in 0..NAME_LEN {
        let val: u8 =
            if idx < upper_bytes.len() {
                upper_bytes[idx]
            } else {
                padding
            };
        enc_bytes[idx*2+1] = ((val & 0xF0) >> 4) + 0x41;
        enc_bytes[idx*2+2] = ((val & 0x0F) >> 0) + 0x41;
    }
    return enc_bytes;
}

#[derive(Default)]
struct Request {
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

fn encode_request(req: &Request) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::with_capacity(REQ_FIXED_LEN);
    buffer.extend_from_slice(&encode_u16_be(req.transaction_id));
    buffer.extend_from_slice(&encode_u16_be(req.flags));
    buffer.extend_from_slice(&encode_u16_be(req.questions));
    buffer.extend_from_slice(&encode_u16_be(req.answers));
    buffer.extend_from_slice(&encode_u16_be(req.authority));
    buffer.extend_from_slice(&encode_u16_be(req.extra));
    buffer.extend_from_slice(&encode_name(&req.user_name));
    buffer.extend_from_slice(&encode_u16_be(req.query_type));
    buffer.extend_from_slice(&encode_u16_be(req.query_class));
    return buffer;
}

fn decode_name(bytes: &[u8]) -> Option<String> {
    if bytes[0] != 0x20 {
        return None
    }
    if bytes[ENC_NAME_LEN-1] != 0x00 {
        return None
    }
    let mut buffer: Vec<u8> = Vec::with_capacity(NAME_LEN);
    for idx in 0..NAME_LEN {
        let mut rune: u8 = 0;
        rune |= ((bytes[2*idx+1] - 0x41) & 0xF) << 4;
        rune |= ((bytes[2*idx+2] - 0x41) & 0xF) << 0;
        if rune != 0x20 && rune != 0x00 {
            buffer.push(rune);
        } else {
            break;
        }
    }
    let name = String::from_utf8(buffer).unwrap();
    return Some(name);
}

fn decode_mac(bytes: &[u8]) -> Option<String> {
    let mac: String = format!("{:02X}-{:02X}-{:02X}-{:02X}-{:02X}-{:02X}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]);
    return Some(mac);
}

#[derive(Default)]
struct Response {
    transaction_id: u16,
    flags: u16,
    questions: u16,
    answers: u16,
    authority: u16,
    extra: u16,
    requested_name: String,
    query_type: u16,
    query_class: u16,
    time_to_live: u32,
    record_length: u16,
    name_count: u8,
    name_vec: Vec<String>,
    statistics: String,
}

fn decode_response(bytes: &[u8]) -> Option<Response> {
    if bytes.len() < ENC_NAME_LEN {
        return None;
    }
    let mut nb_res: Response = Response::default();
    nb_res.transaction_id = decode_u16_be(&bytes[0..2]);
    nb_res.flags = decode_u16_be(&bytes[2..4]);
    nb_res.questions = decode_u16_be(&bytes[4..6]);
    nb_res.answers = decode_u16_be(&bytes[6..8]);
    nb_res.authority = decode_u16_be(&bytes[8..10]);
    nb_res.extra = decode_u16_be(&bytes[10..12]);
    nb_res.requested_name = decode_name(&bytes[12..46]).unwrap();
    nb_res.query_type = decode_u16_be(&bytes[46..48]);
    nb_res.query_class = decode_u16_be(&bytes[48..50]);
    nb_res.time_to_live = decode_u32_be(&bytes[50..54]);
    nb_res.record_length = decode_u16_be(&bytes[54..56]);
    nb_res.name_count = decode_u8_be(&bytes[56..57]);
    for idx in 0..nb_res.name_count {
        let start = RES_FIXED_LEN + HOST_NAME_LEN * (idx as usize);
        let finish = start + 15;
        let cow_text = String::from_utf8_lossy(&bytes[start..finish]);
        nb_res.name_vec.push(cow_text.into_owned());
    }
    {
        let start = RES_FIXED_LEN + HOST_NAME_LEN * (nb_res.name_count as usize);
        let finish = bytes.len() - 1;
        nb_res.statistics = decode_mac(&bytes[start..finish]).unwrap();
    }
    return Some(nb_res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_name_test() {
        {
            let bytes = encode_name("0123456");
            let text = String::from_utf8_lossy(&bytes);
            assert_eq!(text, "\x20DADBDCDDDEDFDGCACACACACACACACACA\0");
        }
        {
            let bytes = encode_name("0123456789abcdefghijklmn");
            let text = String::from_utf8_lossy(&bytes);
            assert_eq!(text, "\x20DADBDCDDDEDFDGDHDIDJEBECEDEEEFEG\0");
        }
    }

    #[test]
    fn nbstat_request_test() {
        let nb_req = Request{
            transaction_id: 0x1337,
            flags: 0x0000,
            questions: 1,
            answers: 0,
            authority: 0,
            extra: 0,
            user_name: "0123456".to_string(),
            query_type: 0x0021,
            query_class: 0x0001,
        };
        let req_buf: Vec<u8> = encode_request(&nb_req);
        assert_eq!(req_buf.len(), 50);
        assert_eq!(req_buf[0..2], [0x13, 0x37]);
        assert_eq!(req_buf[2..4], [0x00, 0x00]);
        assert_eq!(req_buf[4..6], [0x00, 0x01]);
        assert_eq!(req_buf[6..8], [0x00, 0x00]);
        assert_eq!(req_buf[8..10], [0x00, 0x00]);
        assert_eq!(req_buf[10..12], [0x00, 0x00]);
        assert_eq!(req_buf[46..48], [0x00, 0x21]);
        assert_eq!(req_buf[48..50], [0x00, 0x01]);
    }

    #[test]
    fn decode_name_test() {
        {
            let bytes: &[u8] = "\x20DADBDCDDDEDFDGCACACACACACACACACA\0".as_bytes();
            let text = decode_name(bytes).unwrap();
            assert_eq!(&text, "0123456")
        }
        {
            let bytes: &[u8] = "\x20DADBDCDDDEDFDGDHDIDJEBECEDEEEFEG\0".as_bytes();
            let text = decode_name(bytes).unwrap();
            assert_eq!(&text, "0123456789ABCDEF")
        }
    }

    fn main() {
        let nb_req = Request{
            transaction_id: 0x1337,
            flags: 0x0000,
            questions: 1,
            answers: 0,
            authority: 0,
            extra: 0,
            user_name: "*".to_string(),
            query_type: 0x0021,
            query_class: 0x0001,
        };
        let req_buf: Vec<u8> = encode_request(&nb_req);
        let mut recv_buf: [u8; 1025] = [0; 1025];

        let udp = net::UdpSocket::bind("192.168.1.8:35009").unwrap();
        //let read_timeout = time::Duration::from_secs(1);
        //udp.set_read_timeout(Some(read_timeout));
        //let write_timeout = time::Duration::from_secs(1);
        //udp.set_write_timeout(Some(write_timeout));
        udp.send_to(req_buf.as_slice(), "192.168.1.7:137").unwrap();
        match udp.recv_from(&mut recv_buf) {
            Err(err) => {
                println!("ERR: {}", err)
            },
            Ok((recv_size, recv_addr)) => {
                let nb_res = decode_response(&recv_buf).unwrap();
                println!("RecvSize: {:?}", recv_size);
                println!("RecvAddr: {:?}", recv_addr);
                println!("NameCount: {:?}", nb_res.name_vec.len());
                println!("{}", nb_res.query_type);
                println!("{}", nb_res.query_class);
                println!("{}", nb_res.time_to_live);
                println!("{}", nb_res.record_length);
                for name in nb_res.name_vec {
                    println!("RecvAddr: {:?}", name);
                }
                println!("Statistics: {:?}", nb_res.statistics);
            }
        }
    }
}
