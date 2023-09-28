use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

pub(crate) fn get_file_contents(file_path: PathBuf) -> anyhow::Result<String> {
    let mut file = File::open(file_path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

pub(crate) fn store_file(file_path: PathBuf, file_content: &str) -> anyhow::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(file_content.as_bytes())?;

    Ok(())
}
