pub struct Memory {
    data: [u8; 65536],
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; 65536] }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}