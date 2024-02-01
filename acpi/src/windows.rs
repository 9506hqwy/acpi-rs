use super::RawAcpiData;
use bytes::Bytes;
use windows::core::Error;
use windows::Win32::System::SystemInformation::{
    EnumSystemFirmwareTables, GetSystemFirmwareTable, FIRMWARE_TABLE_PROVIDER,
};

pub const FIRMWARE_TABLE_ACPI: u32 = 0x41435049; // 'ACPI'
pub const FIRMWARE_TABLE_FIRM: u32 = 0x4649524D; // 'FIRM'
pub const FIRMWARE_TABLE_RSMB: u32 = 0x52534D42; // 'RSMB'

pub fn get_raw_table(name: &str) -> Result<RawAcpiData, Error> {
    let sig = u32::from_le_bytes(name.as_bytes().try_into().unwrap());
    let table = get_system_firmware_table(FIRMWARE_TABLE_ACPI, sig)?;
    Ok(RawAcpiData::from(Bytes::from(table)))
}

pub fn table_types() -> Result<Vec<String>, Error> {
    enum_system_firmware_table(FIRMWARE_TABLE_ACPI)
}

fn enum_system_firmware_table(signature: u32) -> Result<Vec<String>, Error> {
    // https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-enumsystemfirmwaretables

    let sig = FIRMWARE_TABLE_PROVIDER(signature);

    let size = unsafe { EnumSystemFirmwareTables(sig, None) };
    if size == 0 {
        return Err(Error::from_win32());
    }

    let mut buffer = vec![0u8; size as usize];

    let size = unsafe { EnumSystemFirmwareTables(sig, Some(buffer.as_mut_slice())) };
    if size == 0 {
        return Err(Error::from_win32());
    }

    let mut tables = buffer
        .chunks_exact(4)
        .map(|b| String::from_utf8_lossy(b).to_string())
        .collect::<Vec<String>>();

    // NOTE:
    // if the system contains multiple tables with the same name,
    // they are all enumerated with EnumSystemFirmwareTables.
    // However, GetSystemFirmwareTable retrieves only the first table in the list with this name.
    tables.sort();
    tables.dedup();

    Ok(tables)
}

fn get_system_firmware_table(signature: u32, table_id: u32) -> Result<Vec<u8>, Error> {
    // https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemfirmwaretable

    let sig = FIRMWARE_TABLE_PROVIDER(signature);

    let size = unsafe { GetSystemFirmwareTable(sig, table_id, None) };
    if size == 0 {
        return Err(Error::from_win32());
    }

    let mut buffer = vec![0u8; size as usize];

    let size = unsafe { GetSystemFirmwareTable(sig, table_id, Some(buffer.as_mut_slice())) };
    if size == 0 {
        return Err(Error::from_win32());
    }

    Ok(buffer)
}
