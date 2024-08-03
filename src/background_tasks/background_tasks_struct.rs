#[derive(Debug)]
pub(crate) struct BackGroundTasks {
    pub checkexpiration: u64,
}

impl BackGroundTasks {
    pub(crate) fn serialize(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.checkexpiration.to_be_bytes());
        bytes.push(0);
        bytes
    }
}