pub mod setup_struct;
use setup_struct::Setup;
use crate::global::{ FILE_PATH_SETUP, PAGE_SIZE};
use std::fs::File;
use std::io::{self, Write};

pub(crate) fn save_setup(host: &str, port: &u32, max_connections: Option<u16>) -> io::Result<()> {

    println!("host: {}, port: {} conn{}.", host, port, max_connections.unwrap_or(0));

    let data_to_write = Setup {
        host: String::from(host),
        port: *port,
    };

    write_setup_to_file(&data_to_write, FILE_PATH_SETUP)?;
    println!("host: {}, port: {} saved.", data_to_write.host, data_to_write.port);

    Ok(())
}

fn write_setup_to_file(data: &Setup, filepath: &str) -> io::Result<()> {
    let serialized_data: Vec<u8> = data.serialize();
    let padding_len: usize = if serialized_data.len() % PAGE_SIZE == 0 {
        0
    } else {
        PAGE_SIZE - (serialized_data.len() % PAGE_SIZE)
    };
    let mut padded_data: Vec<u8> = serialized_data.clone();
    padded_data.extend(vec![0; padding_len]);
    let mut file: File = File::create(filepath)?;
    let hash: Vec<u8> = data.calculate_hash(&padded_data);
    file.write_all(&hash)?;
    file.write_all(&padded_data)?;
    file.flush()?;

    Ok(())
}