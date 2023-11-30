use std::fs::File;
use std::io;
use std::io::Read;

pub(crate) fn day1() -> io::Result<()> {
    let file_path = "resources/test-input.txt";
    let mut file = File::open(file_path)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    println!("File contents:\n{}", buffer);

    Ok(())
}