pub struct StorageManager {
    disk_image: Vec<u8>,
    iso_image: Vec<u8>,
    disk_sector_size: usize,
    iso_sector_size: usize,
}

impl StorageManager {
    pub fn new() -> Self {
        StorageManager {
            disk_image: Vec::new(),
            iso_image: Vec::new(),
            disk_sector_size: 512,
            iso_sector_size: 2048,
        }
    }

    pub fn load_disk_image(&mut self, data: Vec<u8>) {
        self.disk_image = data;
    }

    pub fn load_iso(&mut self, data: Vec<u8>) {
        self.iso_image = data;
    }

    pub fn read_disk_sector(&self, lba: u32, count: u8) -> Vec<u8> {
        let offset = lba as usize * self.disk_sector_size;
        let length = count as usize * self.disk_sector_size;
        
        if offset + length <= self.disk_image.len() {
            self.disk_image[offset..offset + length].to_vec()
        } else if offset < self.disk_image.len() {
            let mut result = self.disk_image[offset..].to_vec();
            result.resize(length, 0);
            result
        } else {
            vec![0; length]
        }
    }

    pub fn write_disk_sector(&mut self, lba: u32, data: &[u8]) {
        let offset = lba as usize * self.disk_sector_size;
        if offset + data.len() <= self.disk_image.len() {
            self.disk_image[offset..offset + data.len()].copy_from_slice(data);
        }
    }

    pub fn read_iso_sector(&self, lba: u32) -> Vec<u8> {
        let offset = lba as usize * self.iso_sector_size;
        if offset + self.iso_sector_size <= self.iso_image.len() {
            self.iso_image[offset..offset + self.iso_sector_size].to_vec()
        } else if offset < self.iso_image.len() {
            let mut result = self.iso_image[offset..].to_vec();
            result.resize(self.iso_sector_size, 0);
            result
        } else {
            vec![0; self.iso_sector_size]
        }
    }

    pub fn read_iso_boot_sector(&self) -> Vec<u8> {
        if self.iso_image.len() >= 512 {
            self.iso_image[0..512].to_vec()
        } else {
            vec![0; 512]
        }
    }

    pub fn disk_size(&self) -> usize {
        self.disk_image.len()
    }

    pub fn iso_size(&self) -> usize {
        self.iso_image.len()
    }

    pub fn has_disk(&self) -> bool {
        !self.disk_image.is_empty()
    }

    pub fn has_iso(&self) -> bool {
        !self.iso_image.is_empty()
    }
}
