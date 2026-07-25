#[derive(Debug, Clone)]
pub enum Z21CommandSend {
    LanGetSerialNumber,
    LanLogoff,
}

impl Z21CommandSend {
    pub fn create_telegram(&self) -> Vec<u8> {
        match self {
            Self::LanGetSerialNumber => vec![0x04, 0x00, 0x10, 0x00],
            Self::LanLogoff => vec![0x04, 0x00, 0x30, 0x00],
        }
    }
}