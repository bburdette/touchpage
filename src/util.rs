use failure::Error as FError;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn load_string(file_name: &str) -> Result<String, FError> {
  let path = &Path::new(&file_name);
  let mut inf = File::open(path)?;
  let mut result = String::new();
  inf.read_to_string(&mut result)?;
  Ok(result)
}

pub fn write_string(text: &str, file_name: &str) -> Result<(), FError> {
  let path = &Path::new(&file_name);
  let mut inf = File::create(path)?;
  inf.write(text.as_bytes())?;
  Ok(())
}
