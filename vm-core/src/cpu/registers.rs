#[derive(Clone, Copy, Debug)]
pub struct Registers {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub esi: u32,
    pub edi: u32,
    pub ebp: u32,
    pub esp: u32,
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
            esi: 0,
            edi: 0,
            ebp: 0,
            esp: 0xFFFE,
            cs: 0xF000,
            ds: 0x0000,
            es: 0x0000,
            fs: 0x0000,
            gs: 0x0000,
            ss: 0x0000,
        }
    }

    pub fn al(&self) -> u8 { self.eax as u8 }
    pub fn ah(&self) -> u8 { (self.eax >> 8) as u8 }
    pub fn ax(&self) -> u16 { self.eax as u16 }
    
    pub fn bl(&self) -> u8 { self.ebx as u8 }
    pub fn bh(&self) -> u8 { (self.ebx >> 8) as u8 }
    pub fn bx(&self) -> u16 { self.ebx as u16 }
    
    pub fn cl(&self) -> u8 { self.ecx as u8 }
    pub fn ch(&self) -> u8 { (self.ecx >> 8) as u8 }
    pub fn cx(&self) -> u16 { self.ecx as u16 }
    
    pub fn dl(&self) -> u8 { self.edx as u8 }
    pub fn dh(&self) -> u8 { (self.edx >> 8) as u8 }
    pub fn dx(&self) -> u16 { self.edx as u16 }

    pub fn set_al(&mut self, v: u8) { self.eax = (self.eax & 0xFFFFFF00) | v as u32; }
    pub fn set_ah(&mut self, v: u8) { self.eax = (self.eax & 0xFFFF00FF) | ((v as u32) << 8); }
    pub fn set_ax(&mut self, v: u16) { self.eax = (self.eax & 0xFFFF0000) | v as u32; }

    pub fn set_bl(&mut self, v: u8) { self.ebx = (self.ebx & 0xFFFFFF00) | v as u32; }
    pub fn set_bh(&mut self, v: u8) { self.ebx = (self.ebx & 0xFFFF00FF) | ((v as u32) << 8); }
    pub fn set_bx(&mut self, v: u16) { self.ebx = (self.ebx & 0xFFFF0000) | v as u32; }

    pub fn set_cl(&mut self, v: u8) { self.ecx = (self.ecx & 0xFFFFFF00) | v as u32; }
    pub fn set_ch(&mut self, v: u8) { self.ecx = (self.ecx & 0xFFFF00FF) | ((v as u32) << 8); }
    pub fn set_cx(&mut self, v: u16) { self.ecx = (self.ecx & 0xFFFF0000) | v as u32; }

    pub fn set_dl(&mut self, v: u8) { self.edx = (self.edx & 0xFFFFFF00) | v as u32; }
    pub fn set_dh(&mut self, v: u8) { self.edx = (self.edx & 0xFFFF00FF) | ((v as u32) << 8); }
    pub fn set_dx(&mut self, v: u16) { self.edx = (self.edx & 0xFFFF0000) | v as u32; }

    pub fn get_reg8(&self, reg: u8) -> u8 {
        match reg {
            0 => self.al(),
            1 => self.cl(),
            2 => self.dl(),
            3 => self.bl(),
            4 => self.ah(),
            5 => self.ch(),
            6 => self.dh(),
            7 => self.bh(),
            _ => 0,
        }
    }

    pub fn set_reg8(&mut self, reg: u8, value: u8) {
        match reg {
            0 => self.set_al(value),
            1 => self.set_cl(value),
            2 => self.set_dl(value),
            3 => self.set_bl(value),
            4 => self.set_ah(value),
            5 => self.set_ch(value),
            6 => self.set_dh(value),
            7 => self.set_bh(value),
            _ => {}
        }
    }

    pub fn get_reg16(&self, reg: u8) -> u16 {
        match reg {
            0 => self.ax(),
            1 => self.cx(),
            2 => self.dx(),
            3 => self.bx(),
            4 => self.esp as u16,
            5 => self.ebp as u16,
            6 => self.esi as u16,
            7 => self.edi as u16,
            _ => 0,
        }
    }

    pub fn set_reg16(&mut self, reg: u8, value: u16) {
        match reg {
            0 => self.set_ax(value),
            1 => self.set_cx(value),
            2 => self.set_dx(value),
            3 => self.set_bx(value),
            4 => self.esp = (self.esp & 0xFFFF0000) | value as u32,
            5 => self.ebp = (self.ebp & 0xFFFF0000) | value as u32,
            6 => self.esi = (self.esi & 0xFFFF0000) | value as u32,
            7 => self.edi = (self.edi & 0xFFFF0000) | value as u32,
            _ => {}
        }
    }

    pub fn get_reg32(&self, reg: u8) -> u32 {
        match reg {
            0 => self.eax,
            1 => self.ecx,
            2 => self.edx,
            3 => self.ebx,
            4 => self.esp,
            5 => self.ebp,
            6 => self.esi,
            7 => self.edi,
            _ => 0,
        }
    }

    pub fn set_reg32(&mut self, reg: u8, value: u32) {
        match reg {
            0 => self.eax = value,
            1 => self.ecx = value,
            2 => self.edx = value,
            3 => self.ebx = value,
            4 => self.esp = value,
            5 => self.ebp = value,
            6 => self.esi = value,
            7 => self.edi = value,
            _ => {}
        }
    }
}
