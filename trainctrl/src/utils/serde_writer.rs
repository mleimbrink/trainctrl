/*use std::{error::Error, fs::File, io::BufWriter};

use serde::{Serialize, de::DeserializeOwned};

pub trait SerdeWriter {
    fn load(file_name: &str) -> Result<Self, Box<dyn Error>>
    where Self: DeserializeOwned {
        let file = File::open(file_name)?;
        let data: Self = serde_json::from_reader(file)?;

        Ok(data)
    }

    fn write(&self, file_name: &str) -> Result<(), Box<dyn Error>>
    where Self: Serialize {

        let file = File::create(file_name)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;

        Ok(())
    }
}*/