use crate::{utils::primitives::Primitives, z21_command::Z21Command};

#[derive(Debug, Clone)]
pub enum Z21CommandReceive {
    LanGetSerialNumber(u32),
}

impl Z21CommandReceive {
    pub fn interprete_telegram(telegram: &[u8]) -> Option<Self> {

        let mut r = 0..1;
        let len = Primitives::get_u16(telegram, &mut r) as u8;
        let header = Primitives::get_u16(telegram, &mut r);

        match (header, len) {
            (0x10, 0x08) => Some(Self::LanGetSerialNumber(Primitives::get_u32(telegram, &mut r))),
            _ => None,
        }
    }
}