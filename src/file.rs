use std::{fs::File, io::Read, path::PathBuf};

pub(crate) fn get_file_contents(file_path: PathBuf) -> anyhow::Result<String> {
    let mut file = File::open(file_path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}
