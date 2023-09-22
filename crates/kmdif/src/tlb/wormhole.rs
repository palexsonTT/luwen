use crate::{
    tlb::{MemoryType, TlbInfo},
    DeviceTlbInfo, PciDevice, PciError, Tlb,
};

use super::Ordering;

#[bitfield_struct::bitfield(u64)]
pub struct Tlb1M {
    local_offset: u16,
    #[bits(6)]
    x_end: u8,
    #[bits(6)]
    y_end: u8,
    #[bits(6)]
    x_start: u8,
    #[bits(6)]
    y_start: u8,
    #[bits(1)]
    noc_sel: u8,
    mcast: bool,
    #[bits(2)]
    ordering: u8,
    linked: bool,
    #[bits(19)]
    padding: u64,
}

impl From<Tlb> for Tlb1M {
    fn from(value: Tlb) -> Self {
        Self::new()
            .with_local_offset(value.local_offset as u16)
            .with_x_end(value.x_end)
            .with_y_end(value.y_end)
            .with_x_start(value.x_start)
            .with_y_start(value.y_start)
            .with_noc_sel(value.noc_sel)
            .with_mcast(value.mcast)
            .with_ordering(value.ordering.into())
            .with_linked(value.linked)
    }
}

impl From<Tlb1M> for Tlb {
    fn from(value: Tlb1M) -> Self {
        Tlb {
            local_offset: value.local_offset() as u64,
            x_end: value.x_end(),
            y_end: value.y_end(),
            x_start: value.x_start(),
            y_start: value.y_start(),
            noc_sel: value.noc_sel(),
            mcast: value.mcast(),
            ordering: Ordering::from(value.ordering()),
            linked: value.linked(),
        }
    }
}

#[bitfield_struct::bitfield(u64)]
pub struct Tlb2M {
    #[bits(15)]
    local_offset: u16,
    #[bits(6)]
    x_end: u8,
    #[bits(6)]
    y_end: u8,
    #[bits(6)]
    x_start: u8,
    #[bits(6)]
    y_start: u8,
    #[bits(1)]
    noc_sel: u8,
    mcast: bool,
    #[bits(2)]
    ordering: u8,
    linked: bool,
    #[bits(20)]
    padding: u64,
}

impl From<Tlb> for Tlb2M {
    fn from(value: Tlb) -> Self {
        Self::new()
            .with_local_offset(value.local_offset as u16)
            .with_x_end(value.x_end)
            .with_y_end(value.y_end)
            .with_x_start(value.x_start)
            .with_y_start(value.y_start)
            .with_noc_sel(value.noc_sel)
            .with_mcast(value.mcast)
            .with_ordering(value.ordering.into())
            .with_linked(value.linked)
    }
}

impl From<Tlb2M> for Tlb {
    fn from(value: Tlb2M) -> Self {
        Tlb {
            local_offset: value.local_offset() as u64,
            x_end: value.x_end(),
            y_end: value.y_end(),
            x_start: value.x_start(),
            y_start: value.y_start(),
            noc_sel: value.noc_sel(),
            mcast: value.mcast(),
            ordering: Ordering::from(value.ordering()),
            linked: value.linked(),
        }
    }
}

#[bitfield_struct::bitfield(u64)]
pub struct Tlb16M {
    #[bits(12)]
    local_offset: u16,
    #[bits(6)]
    x_end: u8,
    #[bits(6)]
    y_end: u8,
    #[bits(6)]
    x_start: u8,
    #[bits(6)]
    y_start: u8,
    #[bits(1)]
    noc_sel: u8,
    mcast: bool,
    #[bits(2)]
    ordering: u8,
    linked: bool,
    #[bits(23)]
    padding: u64,
}

impl From<Tlb16M> for Tlb {
    fn from(value: Tlb16M) -> Self {
        Tlb {
            local_offset: value.local_offset() as u64,
            x_end: value.x_end() as u8,
            y_end: value.y_end() as u8,
            x_start: value.x_start() as u8,
            y_start: value.y_start() as u8,
            noc_sel: value.noc_sel(),
            mcast: value.mcast(),
            ordering: Ordering::from(value.ordering()),
            linked: value.linked(),
        }
    }
}

impl From<Tlb> for Tlb16M {
    fn from(value: Tlb) -> Self {
        Self::new()
            .with_local_offset(value.local_offset as u16)
            .with_x_end(value.x_end)
            .with_y_end(value.y_end)
            .with_x_start(value.x_start)
            .with_y_start(value.y_start)
            .with_noc_sel(value.noc_sel)
            .with_mcast(value.mcast)
            .with_ordering(value.ordering.into())
            .with_linked(value.linked)
    }
}

// For WH we have 156 1MB TLBS, 10 2MB TLBS and 20 16 MB TLBs
// For now I'll allow all to be programmed, but I'll only use tlb 20
pub fn setup_tlb(
    device: &mut PciDevice,
    tlb_index: u32,
    mut tlb: Tlb,
) -> Result<(u64, u64), PciError> {
    const TLB_CONFIG_BASE: u64 = 0x1FC00000;

    const TLB_COUNT_1M: u64 = 156;
    const TLB_COUNT_2M: u64 = 10;
    const _TLB_COUNT_16M: u64 = 20;

    const _TLB_INDEX_1M: u64 = 0;
    const _TLB_INDEX_2M: u64 = TLB_COUNT_1M;
    const _TLB_INDEX_16M: u64 = TLB_COUNT_1M + TLB_COUNT_2M;

    const TLB_BASE_1M: u64 = 0;
    const TLB_BASE_2M: u64 = TLB_COUNT_1M * (1 << 20);
    const TLB_BASE_16M: u64 = TLB_BASE_2M + TLB_COUNT_2M * (1 << 21);

    let tlb_config_addr = TLB_CONFIG_BASE + (tlb_index as u64 * 8);

    let (tlb_value, mmio_addr, size, addr_offset) = match tlb_index {
        0..=155 => {
            let size = 1 << 20;
            let tlb_address = tlb.local_offset as u64 / size;
            let local_offset = tlb.local_offset % size;

            tlb.local_offset = tlb_address;
            (
                Tlb1M::from(tlb).0,
                TLB_BASE_1M + size * tlb_index as u64,
                size,
                local_offset,
            )
        }
        156..=165 => {
            let size = 1 << 21;
            let tlb_address = tlb.local_offset as u64 / size;
            let local_offset = tlb.local_offset % size;

            tlb.local_offset = tlb_address;
            (
                Tlb2M::from(tlb).0,
                TLB_BASE_2M + size * (tlb_index - 156) as u64,
                size,
                local_offset,
            )
        }
        166..=185 => {
            let size = 1 << 24;
            let tlb_address = tlb.local_offset as u64 / size;
            let local_offset = tlb.local_offset % size;

            tlb.local_offset = tlb_address;
            (
                Tlb16M::from(tlb).0,
                TLB_BASE_16M + size * (tlb_index - 166) as u64,
                size,
                local_offset,
            )
        }
        _ => {
            panic!("TLB index out of range");
        }
    };

    device.write32(tlb_config_addr as u32, (tlb_value & 0xFFFF_FFFF) as u32)?;
    device.write32(
        tlb_config_addr as u32 + 4,
        ((tlb_value >> 32) & 0xFFFF_FFFF) as u32,
    )?;

    Ok((mmio_addr + addr_offset, size - addr_offset))
}

pub fn get_tlb(device: &PciDevice, tlb_index: u32) -> Result<Tlb, PciError> {
    const TLB_CONFIG_BASE: u32 = 0x1FC00000;
    let tlb_config_addr = TLB_CONFIG_BASE + (tlb_index * 8);

    let tlb = ((device.read32(tlb_config_addr + 4)? as u64) << 32)
        | device.read32(tlb_config_addr as u32)? as u64;

    let output = match tlb_index {
        0..=155 => Tlb1M::from(tlb).into(),
        156..=165 => Tlb2M::from(tlb).into(),
        166..=185 => Tlb16M::from(tlb).into(),
        _ => {
            panic!("TLB index out of range");
        }
    };

    Ok(output)
}

pub fn tlb_info(device: &PciDevice) -> DeviceTlbInfo {
    const TLB_COUNT_1M: u64 = 156;
    const TLB_COUNT_2M: u64 = 10;
    const TLB_COUNT_16M: u64 = 20;

    DeviceTlbInfo {
        device_id: device.id as u32,
        total_count: 186,
        tlb_config: vec![
            TlbInfo {
                count: TLB_COUNT_1M,
                size: 1 << 20,
                memory_type: MemoryType::Uc,
            },
            TlbInfo {
                count: TLB_COUNT_2M,
                size: 1 << 21,
                memory_type: MemoryType::Uc,
            },
            TlbInfo {
                count: TLB_COUNT_16M,
                size: 1 << 24,
                memory_type: MemoryType::Uc,
            },
        ],
    }
}
