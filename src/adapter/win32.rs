#![feature(libc)]
extern crate libc;

#[link(name="win_net")]
extern {
    #[repr(C)]
    struct CAdapterInfo {
        ip_address: libc::int32_t,
        ip_mask: libc::int32_t,
        gateway: libc::int32_t,
        mac_address:  libc::int64_t
    }

    #[repr(C)]
    struct CHostInfo {
        ip_address: libc::int32_t,
        mac_address:  libc::int64_t
    }

    fn get_adapter_count() -> libc::uint32_t;
}
