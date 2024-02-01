use super::RawAcpiData;
use bytes::Bytes;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

const BASE_PATH: &str = "/sys/firmware/acpi/tables";

pub fn get_raw_table(name: &str) -> Result<RawAcpiData, Error> {
    let path = PathBuf::from(BASE_PATH).join(name);
    let table = fs::read(path)?;
    Ok(RawAcpiData::from(Bytes::from(table)))
}

pub fn table_types() -> Result<Vec<String>, Error> {
    let mut tables = vec![];
    for entry in fs::read_dir(BASE_PATH)? {
        let entry = entry?;
        if entry.path().is_file() {
            let file_name = entry
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            tables.push(file_name);
        }
    }

    tables.sort();

    Ok(tables)
}
