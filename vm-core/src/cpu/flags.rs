pub struct Flags {
    value: u32,
}

impl Flags {
    pub fn new() -> Self {
        Flags { value: 0x00000002 } // Reserved bit always set
    }

    pub fn raw(&self) -> u32 { self.value }
    pub fn set_raw(&mut self, v: u32) { self.value = v; }

    // Carry Flag
    pub fn cf(&self) -> bool { (self.value & 0x0001) != 0 }
    pub fn set_cf(&mut self, v: bool) {
        if v { self.value |= 0x0001; } else { self.value &= !0x0001; }
    }

    // Parity Flag
    pub fn pf(&self) -> bool { (self.value & 0x0004) != 0 }
    pub fn set_pf(&mut self, v: bool) {
        if v { self.value |= 0x0004; } else { self.value &= !0x0004; }
    }

    // Auxiliary Carry Flag
    pub fn af(&self) -> bool { (self.value & 0x0010) != 0 }
    pub fn set_af(&mut self, v: bool) {
        if v { self.value |= 0x0010; } else { self.value &= !0x0010; }
    }

    // Zero Flag
    pub fn zf(&self) -> bool { (self.value & 0x0040) != 0 }
    pub fn set_zf(&mut self, v: bool) {
        if v { self.value |= 0x0040; } else { self.value &= !0x0040; }
    }

    // Sign Flag
    pub fn sf(&self) -> bool { (self.value & 0x0080) != 0 }
    pub fn set_sf(&mut self, v: bool) {
        if v { self.value |= 0x0080; } else { self.value &= !0x0080; }
    }

    // Trap Flag
    pub fn tf(&self) -> bool { (self.value & 0x0100) != 0 }
    pub fn set_tf(&mut self, v: bool) {
        if v { self.value |= 0x0100; } else { self.value &= !0x0100; }
    }

    // Interrupt Flag
    pub fn if_(&self) -> bool { (self.value & 0x0200) != 0 }
    pub fn set_if(&mut self, v: bool) {
        if v { self.value |= 0x0200; } else { self.value &= !0x0200; }
    }

    // Direction Flag
    pub fn df(&self) -> bool { (self.value & 0x0400) != 0 }
    pub fn set_df(&mut self, v: bool) {
        if v { self.value |= 0x0400; } else { self.value &= !0x0400; }
    }

    // Overflow Flag
    pub fn of(&self) -> bool { (self.value & 0x0800) != 0 }
    pub fn set_of(&mut self, v: bool) {
        if v { self.value |= 0x0800; } else { self.value &= !0x0800; }
    }

    pub fn update_flags_8(&mut self, result: u8) {
        self.set_zf(result == 0);
        self.set_sf((result & 0x80) != 0);
        // Parity: even number of 1 bits in low byte
        let parity = result.count_ones() % 2 == 0;
        self.set_pf(parity);
    }

    pub fn update_flags_16(&mut self, result: u16) {
        self.set_zf(result == 0);
        self.set_sf((result & 0x8000) != 0);
        let parity = (result as u8).count_ones() % 2 == 0;
        self.set_pf(parity);
    }

    pub fn update_flags_32(&mut self, result: u32) {
        self.set_zf(result == 0);
        self.set_sf((result & 0x80000000) != 0);
        let parity = (result as u8).count_ones() % 2 == 0;
        self.set_pf(parity);
    }
}
