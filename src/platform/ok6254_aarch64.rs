use crate::{arch::zone::HvArchZoneConfig, config::*};

pub const ROOT_ZONE_DTB_ADDR: u64 = 0x80000000;  // 设备树地址
pub const ROOT_ZONE_KERNEL_ADDR: u64 = 0x80400000;  // 内核加载地址
pub const ROOT_ZONE_ENTRY: u64 = 0x90400000;  // 内核入口地址
pub const ROOT_ZONE_CPUS: u64 = (1 << 0) | (1 << 1); // cpus = 3

pub const ROOT_ZONE_NAME: &str = "root-linux";

// pub const ROOT_ZONE_MEMORY_REGIONS: [HvConfigMemoryRegion; 3] = [
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_RAM,
//         physical_start: 0xa0000000,
//         virtual_start: 0x50000000,
//         size: 0xf0000000,
//     }, // ram
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0x30000000,
//         virtual_start: 0x30000000,
//         size: 0x400000,
//     }, // bus@30000000
//     HvConfigMemoryRegion {
//         mem_type: MEM_TYPE_IO,
//         physical_start: 0x30800000,
//         virtual_start: 0x30800000,
//         size: 0x400000,
//     },
// ];

pub const ROOT_ZONE_MEMORY_REGIONS: [HvConfigMemoryRegion; 3] = [
    // RAM区域：覆盖DTS中memory节点定义的物理内存，排除保留区域
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_RAM,
        physical_start: 0x80000000,   // 起始地址对齐DTS
        virtual_start: 0x80000000,    // 直接映射
        size: 0x80000000,             // 0x80000000 ~ 0xffffffff
    },
    // IO区域1
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x00000000,  
        virtual_start: 0x00000000,
        size: 0x30000000,             // 0x0 ~ 0x30000000
    },
    // IO区域2
    HvConfigMemoryRegion {
        mem_type: MEM_TYPE_IO,
        physical_start: 0x30000000,
        virtual_start: 0x30000000,
        size: 0x50000000,             // 0x30000000 ~ 0x80000000
    },
];

pub const ROOT_ZONE_IRQS: [u32; 19] = [
    36, 52, 55, 59, 64, 67, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 150, 151, 152,
];

pub const ROOT_ARCH_ZONE_CONFIG: HvArchZoneConfig = HvArchZoneConfig {
    gicd_base: 0x1800000,    // 直接使用设备树中的 GICD 基地址
    gicd_size: 0x10000,      // 64KB
    gicr_base: 0x1880000,    // 直接使用设备树中的 GICR 基地址
    gicr_size: 0xc0000,      // 768KB
};


// make LOG=trace all
// make LOG=LOG_WARN all
