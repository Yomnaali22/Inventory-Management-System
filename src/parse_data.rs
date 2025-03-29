use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::to_string_pretty;
use std::fmt::Debug;
use std::fs::{File, read_to_string};
use std::io::{Result, Write};
pub struct JsonData;

impl JsonData {
    pub fn process<T: DeserializeOwned + Debug + Serialize>(path: &str) -> Result<T> {
        let data = read_to_string(path)?;
        let inventory: T = serde_json::from_str(&data)?;
        Ok(inventory)
    }

    pub fn writes<T: DeserializeOwned + Debug + Serialize>(path: &str, data: &T) -> Result<()> {
        let update_json = to_string_pretty(data)?;
        let mut file = File::create(path)?;
        file.write_all(update_json.as_bytes())?;
        Ok(())
    }
}
