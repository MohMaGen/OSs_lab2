use sysinfo::System;

#[derive(Debug, PartialEq)]
pub enum MemoryUnit {
    Byte(u64),
    KByte(f64),
    MByte(f64),
    GByte(f64),
    TByte(f64),
}

impl MemoryUnit {
    pub fn new(bytes: u64) -> Self {
        Self::Byte(bytes).normalized()
    }

    pub fn normalized(&self) -> Self {
        let bytes = self.get_bytes();

        if bytes > 600_000_000_000 {
            MemoryUnit::TByte(bytes as f64 / 1_000_000_000_000.)
        } else if bytes > 600_000_000 {
            MemoryUnit::GByte(bytes as f64 / 1_000_000_000.)
        } else if bytes > 600_000 {
            MemoryUnit::MByte(bytes as f64 / 1_000_000.)
        } else if bytes > 600 {
            MemoryUnit::KByte(bytes as f64 / 1_000.)
        } else {
            MemoryUnit::Byte(bytes)
        }
    }

    pub fn get_bytes(&self) -> u64 {
        match self {
            MemoryUnit::Byte(v) => *v,
            MemoryUnit::KByte(v) => *v as u64 * 1_000,
            MemoryUnit::MByte(v) => *v as u64 * 1_000_000,
            MemoryUnit::GByte(v) => *v as u64 * 1_000_000_000,
            MemoryUnit::TByte(v) => *v as u64 * 1_000_000_000_000,
        }
    }
}

impl std::fmt::Display for MemoryUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryUnit::Byte(v) => write!(f, "{v} B"),
            MemoryUnit::KByte(v) => write!(f, "{v:.2} KB"),
            MemoryUnit::MByte(v) => write!(f, "{v:.2} MB"),
            MemoryUnit::GByte(v) => write!(f, "{v:.2} GB"),
            MemoryUnit::TByte(v) => write!(f, "{v:.2} TB"),
        }
    }
}

pub fn get_ram_data(system: &System) -> (MemoryUnit, MemoryUnit, MemoryUnit, MemoryUnit) {
    (
        MemoryUnit::new(system.total_memory()),
        MemoryUnit::new(system.used_memory()),
        MemoryUnit::new(system.total_swap()),
        MemoryUnit::new(system.used_swap()),
    )
}
