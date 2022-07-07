use crate::boot_param::boot_params_interface::{ExtListHeader, MemoryMap};
use crate::{BootParamsInterface, INFO};

pub mod boot_params_interface;

const MEM_SIG: u64 = 5064013;

// 解析启动信息
pub fn parse_params(bpi: *const BootParamsInterface) {
    let boot_params_interface = unsafe { &*(bpi as *const BootParamsInterface) };
    let ext_list_ptr = boot_params_interface.ext_list;
    INFO!("!{:#x}", ext_list_ptr as usize);
    let ext_list_header = unsafe { &*(ext_list_ptr as *const ExtListHeader) };
    // 判断ext_list_header是哪一种类型
    loop {
        INFO!("Signature: {}", ext_list_header.signature);
        INFO!("length: {}", ext_list_header.length);
        if ext_list_header.signature == MEM_SIG {
            //内存布局节点
            let ext_mem_map = unsafe { &*(ext_list_ptr as *const MemoryMap) };
            INFO!("MapCount: {}", ext_mem_map.map_count);
            INFO!("Signature: {}", ext_mem_map.header.signature);
        }
        break;
    }
}
