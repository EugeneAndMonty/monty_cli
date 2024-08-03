use sha2::{Digest, Sha256};

#[derive(Debug)]
pub(crate) struct Setup {
    pub host: String,
    pub port: u32,
}

impl Setup {
    pub(crate) fn serialize(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(self.host.as_bytes());
        bytes.push(0);
        bytes.extend(&self.port.to_be_bytes()); // Serialize the port as bytes
        bytes
    }

    pub(crate) fn calculate_hash(&self, serialized_data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(serialized_data);
        hasher.finalize().to_vec()
    }

}