pub enum Z21Command {
    LanGetSerialNumber = 0x10,
    LanLogoff = 0x30,
}

impl From<Z21Command> for u16 {
    fn from(value: Z21Command) -> Self {
        value as Self
    }
}