extern crate libc;

#[link(name="arp", kind = "static")]
extern {
    #[repr(C)]
    struct CppMacAddress {
        addr: [libc::uint_8; 6],
    }

    fn cpp_send_arp(remote_ip: uint32_t) -> CppArpInfo;
}

fn send_arp(remote_ip: u32) -> [u8; 6] {
    let mac_addr = cpp_send_arp(remote_ip);
    return mac_addr.addr;
}

fn send_arp(remote_ip: &str) -> [u8; 6]  {
    return send_arp();
}
