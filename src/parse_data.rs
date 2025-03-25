use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{from_reader, to_string_pretty};
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Result, Write};
pub struct JsonData;

impl JsonData {
    pub fn process<T: DeserializeOwned + Debug + Serialize>(path: &str) -> Result<T> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data = from_reader(reader)?;
        Ok(data)
    }

    pub fn writes<T: DeserializeOwned + Debug + Serialize>(path: &str, data: &T) -> Result<String> {
        let update_json = to_string_pretty(data)?;
        // Open the file for writing (truncate = overwrite)
        let file = OpenOptions::new().write(true).truncate(true).open(path)?;

        let mut writer = BufWriter::new(file);
        writer.write(update_json.as_bytes())?;
        writer.flush()?;

        Ok("n".to_string())
    }
}
