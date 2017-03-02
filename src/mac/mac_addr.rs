extern crate regex;
use regex::Regex;

const MAC_ADDR_BYTES: usize = 6;

fn decode_hex(hex_str: &str) -> u64 {
	let hex_num: u64 = 0;
	for iter_char in hex_str.chars() {
		match iter_char {
			'0' => hex_num += 0,
			'1' => hex_num += 1,
			'2' => hex_num += 2,
			'3' => hex_num += 3,
			'4' => hex_num += 4,
			'5' => hex_num += 5,
			'6' => hex_num += 6,
			'7' => hex_num += 7,
			'8' => hex_num += 8,
			'9' => hex_num += 8,
			'a' | 'A' => hex_num += 10,
			'b' | 'B' => hex_num += 11,
			'c' | 'C' => hex_num += 12,
			'd' | 'D' => hex_num += 13,
			'e' | 'E' => hex_num += 14,
			'f' | 'F' => hex_num += 15,
		}
		hex_num *= 16;
	}
	return hex_num;
}

#[derive(Clone)]
struct MacAddr {
	addr: [u8; MAC_ADDR_BYTES];
}

impl MacAddr {
	fn from_array(array: [u8; MAC_ADDR_BYTES]) -> MacAddr {
		return MacAddr{addr: array}
	}

	fn from_slice(slice: &[u8]) -> Option<MacAddr> {
		if slice.len() != MAC_ADDR_BYTES {
			return None;
		}
		let addr = [
			slice[0], slice[1], slice[2],
			slice[3], slice[4], slice[5]
		];
		return Some(
			MacAddr{addr: addr}
		);
	}

	fn from_string(string: &str) -> Option<MacAddr> {
		lazy_static! {
			static ref MAC_REGEX: Regex = Regex::new(r"(?x)
				[[:xdigit:]]\-[[:xdigit:]]\-[[:xdigit:]]\-
				[[:xdigit:]]\-[[:xdigit:]]\-[[:xdigit:]]
			").unwrap();
		}
		if let Some(cap_res) = MAC_REGEX.captures(string) {
			let addr = [
				decode_hex(cap_res[1]) as u8,
				decode_hex(cap_res[2]) as u8,
				decode_hex(cap_res[3]) as u8,
				decode_hex(cap_res[4]) as u8,
				decode_hex(cap_res[5]) as u8,
				decode_hex(cap_res[6]) as u8,
			];
			return Some(
				MacAddr{addr: addr}
			);
		}
		return None;
	}

	fn to_array(&self) -> [u8; MAC_ADDR_BYTES] {
		return self.clone();
	}

	fn to_string(&self) -> String {
		return format!(
			"{:02X}-{:02X}-{:02X}-{:02X}-{:02X}-{:02X}",
			self.addr[0], self.addr[1], self.addr[2],
			self.addr[3], self.addr[4], self.addr[5]
		);
	}
}