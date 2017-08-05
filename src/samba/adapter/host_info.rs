extern crate libc;

#[link(name = "snappy")]
extern {
    struct AdapterInfo {
      ip_address: uint32_t,
      ip_mask: uint32_t,
      gateway: uint32_t,
      mac_address: [uint8_t; 6],
    }

    struct AdapterInfoGroup {
      count: uint32_t;
      pointer: *mut AdapterInfo;
    }

    fn query_adapter_info() -> *const AdapterInfoGroup;

    fn free_adapter_info(group: *const AdapterInfoGroup);
}

fn enum_adapter_info() -> Result<Vec<AdapterInfo>> {
    let group_vec = Vec<AdapterInfo>::new();
    unsafe {
        let group_ptr: *mut AdapterInfoGroup = query_adapter_info();
        if group_ptr.is_null() {
            return Err();
        }
        let group_obj: AdapterInfo = group_ptr.as_mut().unwrap();
        for idx in 0..groupObj.count {
            let info_ptr: *mut AdapterInfo = group_obj.pointer.offset(idx);
            if !info_obj.is_null() {
                let info_obj: AdapterInfo = info_ptr.as_mut().unwrap();
                group_vec.push(info_obj);
            }
        }
        free_adapter_info(AdapterInfoGroup);
    }
    return Ok(group_vec);
}
