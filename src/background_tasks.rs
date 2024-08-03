use std::io;
use crate::global::{PAGE_SIZE, FILE_PATH_BACKGROUND_TASKS};
use self::background_tasks_struct::BackGroundTasks;
mod background_tasks_struct;
use std::fs::File;
use std::io::Write;

pub(crate) fn save_background_tasks_setup(checkexpiration: u64) -> io::Result<()> {
    let data_to_write: BackGroundTasks = BackGroundTasks {
        checkexpiration,
    };
    write_background_tasks_setup_to_file(&data_to_write, FILE_PATH_BACKGROUND_TASKS)?;
    println!("Engine will check if keys expired every {} sec", data_to_write.checkexpiration);
    Ok(())
}

fn write_background_tasks_setup_to_file(data: &BackGroundTasks, filepath: &str) -> io::Result<()> {
    let serialized_data: Vec<u8> = data.serialize();
    let padding_len: usize = if serialized_data.len() % PAGE_SIZE == 0 {
        0
    } else {
        PAGE_SIZE - (serialized_data.len() % PAGE_SIZE)
    };
    let mut padded_data: Vec<u8> = serialized_data.clone();
    padded_data.extend(vec![0; padding_len]);
    let mut file: File = File::create(filepath)?;
    file.write_all(&padded_data)?;
    file.flush()?;

    Ok(())
}