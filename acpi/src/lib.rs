pub mod error;

#[cfg(target_family = "unix")]
mod unix;
#[cfg(target_family = "windows")]
mod windows;

#[cfg(target_family = "unix")]
pub use self::unix::{get_raw_table, table_types};
#[cfg(target_family = "windows")]
pub use self::windows::{get_raw_table, table_types};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use error::Error;

pub fn get<T>(signature: &str) -> Result<T, Error>
where
    T: From<RawAcpiData>,
{
    let table = get_raw_table(signature)?;
    Ok(T::from(table))
}

// -----------------------------------------------------------------------------------------------

fn extract_string<const N: usize>(value: &mut Bytes) -> String {
    let value_bytes = value.split_to(N);
    let mut v = &value_bytes[..];
    while let Some(b) = v.strip_suffix(&[0]) {
        v = b;
    }
    String::from_utf8_lossy(v).to_string()
}

fn string_to_array<const N: usize>(value: &str) -> [u8; N] {
    let mut v = [value.as_bytes(), &[0u8; N]].concat();
    v.truncate(N);
    <[u8; N]>::try_from(v.as_slice()).unwrap()
}

// -----------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RawAcpiData {
    pub signature: String,
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: String,
    pub oem_table_id: String,
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
    pub acpi_table_data: Bytes,
}

impl From<Bytes> for RawAcpiData {
    fn from(mut buf: Bytes) -> Self {
        let signature = extract_string::<4>(&mut buf);
        let length = buf.get_u32_le();
        let revision = buf.get_u8();
        let checksum = buf.get_u8();
        let oem_id = extract_string::<6>(&mut buf);
        let oem_table_id = extract_string::<8>(&mut buf);
        let oem_revision = buf.get_u32_le();
        let creator_id = buf.get_u32_le();
        let creator_revision = buf.get_u32_le();
        let acpi_table_data = buf.split_off(0);

        RawAcpiData {
            signature,
            length,
            revision,
            checksum,
            oem_id,
            oem_table_id,
            oem_revision,
            creator_id,
            creator_revision,
            acpi_table_data,
        }
    }
}

impl From<RawAcpiData> for Bytes {
    fn from(val: RawAcpiData) -> Self {
        let signature = string_to_array::<4>(&val.signature);
        let oem_id = string_to_array::<6>(&val.oem_id);
        let oem_table_id = string_to_array::<8>(&val.oem_table_id);

        let mut b = BytesMut::with_capacity(val.length as usize);
        b.put_slice(&signature);
        b.put_u32_le(val.length);
        b.put_u8(val.revision);
        b.put_u8(val.checksum);
        b.put_slice(&oem_id);
        b.put_slice(&oem_table_id);
        b.put_u32_le(val.oem_revision);
        b.put_u32_le(val.creator_id);
        b.put_u32_le(val.creator_revision);
        b.put(val.acpi_table_data);
        b.freeze()
    }
}

// -----------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BootGraphicsResource {
    pub signature: String,
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: String,
    pub oem_table_id: String,
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
    pub version: u16,
    pub status: u8,
    pub image_type: u8,
    pub image_address: [u8; 8],
    pub image_offset_x: u32,
    pub image_offset_y: u32,
}

impl From<RawAcpiData> for BootGraphicsResource {
    fn from(mut data: RawAcpiData) -> Self {
        let signature = data.signature;
        let length = data.length;
        let revision = data.revision;
        let checksum = data.checksum;
        let oem_id = data.oem_id;
        let oem_table_id = data.oem_table_id;
        let oem_revision = data.oem_revision;
        let creator_id = data.creator_id;
        let creator_revision = data.creator_revision;
        let version = data.acpi_table_data.get_u16_le();
        let status = data.acpi_table_data.get_u8();
        let image_type = data.acpi_table_data.get_u8();
        let image_address = data.acpi_table_data.split_to(8)[..].try_into().unwrap();
        let image_offset_x = data.acpi_table_data.get_u32_le();
        let image_offset_y = data.acpi_table_data.get_u32_le();

        BootGraphicsResource {
            signature,
            length,
            revision,
            checksum,
            oem_id,
            oem_table_id,
            oem_revision,
            creator_id,
            creator_revision,
            version,
            status,
            image_type,
            image_address,
            image_offset_x,
            image_offset_y,
        }
    }
}

impl From<BootGraphicsResource> for Bytes {
    fn from(val: BootGraphicsResource) -> Self {
        let signature = string_to_array::<4>(&val.signature);
        let oem_id = string_to_array::<6>(&val.oem_id);
        let oem_table_id = string_to_array::<8>(&val.oem_table_id);

        let mut b = BytesMut::with_capacity(val.length as usize);
        b.put_slice(&signature);
        b.put_u32_le(val.length);
        b.put_u8(val.revision);
        b.put_u8(val.checksum);
        b.put_slice(&oem_id);
        b.put_slice(&oem_table_id);
        b.put_u32_le(val.oem_revision);
        b.put_u32_le(val.creator_id);
        b.put_u32_le(val.creator_revision);
        b.put_u16_le(val.version);
        b.put_u8(val.status);
        b.put_u8(val.image_type);
        b.put_slice(&val.image_address);
        b.put_u32_le(val.image_offset_x);
        b.put_u32_le(val.image_offset_y);
        b.freeze()
    }
}

// -----------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MemoryMappedConfiguration {
    pub signature: String,
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: String,
    pub oem_table_id: String,
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
    pub reserved: [u8; 8],
    pub spaces: Vec<MemoryMappedConfigurationSpace>,
}

impl From<RawAcpiData> for MemoryMappedConfiguration {
    fn from(mut data: RawAcpiData) -> Self {
        let signature = data.signature;
        let length = data.length;
        let revision = data.revision;
        let checksum = data.checksum;
        let oem_id = data.oem_id;
        let oem_table_id = data.oem_table_id;
        let oem_revision = data.oem_revision;
        let creator_id = data.creator_id;
        let creator_revision = data.creator_revision;
        let reserved = data.acpi_table_data.split_to(8)[..].try_into().unwrap();
        let mut spaces = vec![];

        for d in data.acpi_table_data.chunks(16) {
            let space = Bytes::from(d.to_owned());
            spaces.push(MemoryMappedConfigurationSpace::from(space));
        }

        MemoryMappedConfiguration {
            signature,
            length,
            revision,
            checksum,
            oem_id,
            oem_table_id,
            oem_revision,
            creator_id,
            creator_revision,
            reserved,
            spaces,
        }
    }
}

impl From<MemoryMappedConfiguration> for Bytes {
    fn from(val: MemoryMappedConfiguration) -> Self {
        let signature = string_to_array::<4>(&val.signature);
        let oem_id = string_to_array::<6>(&val.oem_id);
        let oem_table_id = string_to_array::<8>(&val.oem_table_id);

        let mut b = BytesMut::with_capacity(val.length as usize);
        b.put_slice(&signature);
        b.put_u32_le(val.length);
        b.put_u8(val.revision);
        b.put_u8(val.checksum);
        b.put_slice(&oem_id);
        b.put_slice(&oem_table_id);
        b.put_u32_le(val.oem_revision);
        b.put_u32_le(val.creator_id);
        b.put_u32_le(val.creator_revision);
        b.put_slice(&val.reserved);
        for space in val.spaces {
            b.put(Bytes::from(space));
        }
        b.freeze()
    }
}

// -----------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MemoryMappedConfigurationSpace {
    pub base_address: [u8; 8],
    pub segment_number: u16,
    pub bus_number_start: u8,
    pub bus_number_end: u8,
    pub reserved: [u8; 4],
}

impl From<Bytes> for MemoryMappedConfigurationSpace {
    fn from(mut buf: Bytes) -> Self {
        let base_address: [u8; 8] = buf.split_to(8)[..].try_into().unwrap();
        let segment_number = buf.get_u16_le();
        let bus_number_start = buf.get_u8();
        let bus_number_end = buf.get_u8();
        let reserved = buf.split_to(4)[..].try_into().unwrap();

        MemoryMappedConfigurationSpace {
            base_address,
            segment_number,
            bus_number_start,
            bus_number_end,
            reserved,
        }
    }
}

impl From<MemoryMappedConfigurationSpace> for Bytes {
    fn from(val: MemoryMappedConfigurationSpace) -> Self {
        let mut b = BytesMut::with_capacity(16);
        b.put_slice(&val.base_address);
        b.put_u16_le(val.segment_number);
        b.put_u8(val.bus_number_start);
        b.put_u8(val.bus_number_end);
        b.put_slice(&val.reserved);
        b.freeze()
    }
}

// -----------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SystemManagementModeCommunication {
    pub signature: String,
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: String,
    pub oem_table_id: String,
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
    pub identifier: [u8; 16],
    pub data_offset: u16,
    pub sw_smi_number: u32,
    pub buffer_prt_address: [u8; 8],
    pub invocation_register: Option<[u8; 12]>,
}

impl From<RawAcpiData> for SystemManagementModeCommunication {
    fn from(mut data: RawAcpiData) -> Self {
        let signature = data.signature;
        let length = data.length;
        let revision = data.revision;
        let checksum = data.checksum;
        let oem_id = data.oem_id;
        let oem_table_id = data.oem_table_id;
        let oem_revision = data.oem_revision;
        let creator_id = data.creator_id;
        let creator_revision = data.creator_revision;
        let identifier = data.acpi_table_data.split_to(16)[..].try_into().unwrap();
        let data_offset = data.acpi_table_data.get_u16_le();
        let sw_smi_number = data.acpi_table_data.get_u32_le();
        let buffer_prt_address = data.acpi_table_data.split_to(8)[..].try_into().unwrap();
        let invocation_register = if !data.acpi_table_data.is_empty() {
            Some(data.acpi_table_data.split_to(12)[..].try_into().unwrap())
        } else {
            None
        };

        SystemManagementModeCommunication {
            signature,
            length,
            revision,
            checksum,
            oem_id,
            oem_table_id,
            oem_revision,
            creator_id,
            creator_revision,
            identifier,
            data_offset,
            sw_smi_number,
            buffer_prt_address,
            invocation_register,
        }
    }
}

impl From<SystemManagementModeCommunication> for Bytes {
    fn from(val: SystemManagementModeCommunication) -> Self {
        let signature = string_to_array::<4>(&val.signature);
        let oem_id = string_to_array::<6>(&val.oem_id);
        let oem_table_id = string_to_array::<8>(&val.oem_table_id);

        let mut b = BytesMut::with_capacity(val.length as usize);
        b.put_slice(&signature);
        b.put_u32_le(val.length);
        b.put_u8(val.revision);
        b.put_u8(val.checksum);
        b.put_slice(&oem_id);
        b.put_slice(&oem_table_id);
        b.put_u32_le(val.oem_revision);
        b.put_u32_le(val.creator_id);
        b.put_u32_le(val.creator_revision);
        b.put_slice(&val.identifier);
        b.put_u16_le(val.data_offset);
        b.put_u32_le(val.sw_smi_number);
        b.put_slice(&val.buffer_prt_address);
        if let Some(v) = val.invocation_register {
            b.put_slice(&v);
        }
        b.freeze()
    }
}

// -----------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_acpi_data() {
        let data = RawAcpiData {
            signature: "ABCD".to_string(),
            length: 36,
            revision: 1,
            checksum: 2,
            oem_id: "EF".to_string(),
            oem_table_id: "GHI".to_string(),
            oem_revision: 3,
            creator_id: 4,
            creator_revision: 5,
            acpi_table_data: Bytes::from("JKL"),
        };
        let b = Bytes::from(data.clone());
        let ret = RawAcpiData::from(b);
        assert_eq!(data, ret);
    }

    #[test]
    fn boot_graphics_resource() {
        let data = BootGraphicsResource {
            signature: "ABCD".to_string(),
            length: 36,
            revision: 1,
            checksum: 2,
            oem_id: "EF".to_string(),
            oem_table_id: "GHI".to_string(),
            oem_revision: 3,
            creator_id: 4,
            creator_revision: 5,
            version: 6,
            status: 7,
            image_type: 8,
            image_address: [0, 1, 2, 3, 4, 5, 6, 7],
            image_offset_x: 9,
            image_offset_y: 10,
        };
        let b = Bytes::from(data.clone());
        let raw = RawAcpiData::from(b);
        let ret = BootGraphicsResource::from(raw);
        assert_eq!(data, ret);
    }

    #[test]
    fn memory_mapped_configuration() {
        let data = MemoryMappedConfiguration {
            signature: "ABCD".to_string(),
            length: 36,
            revision: 1,
            checksum: 2,
            oem_id: "EF".to_string(),
            oem_table_id: "GHI".to_string(),
            oem_revision: 3,
            creator_id: 4,
            creator_revision: 5,
            reserved: [0, 1, 2, 3, 4, 5, 6, 7],
            spaces: vec![MemoryMappedConfigurationSpace {
                base_address: [6, 7, 8, 9, 10, 11, 12, 13],
                segment_number: 6,
                bus_number_start: 7,
                bus_number_end: 8,
                reserved: [12, 13, 14, 15],
            }],
        };
        let b = Bytes::from(data.clone());
        let raw = RawAcpiData::from(b);
        let ret = MemoryMappedConfiguration::from(raw);
        assert_eq!(data, ret);
    }

    #[test]
    fn system_management_mode_communication() {
        let data = SystemManagementModeCommunication {
            signature: "ABCD".to_string(),
            length: 36,
            revision: 1,
            checksum: 2,
            oem_id: "EF".to_string(),
            oem_table_id: "GHI".to_string(),
            oem_revision: 3,
            creator_id: 4,
            creator_revision: 5,
            identifier: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            data_offset: 6,
            sw_smi_number: 7,
            buffer_prt_address: [16, 17, 18, 19, 20, 21, 22, 23],
            invocation_register: Some([1u8; 12]),
        };
        let b = Bytes::from(data.clone());
        let raw = RawAcpiData::from(b);
        let ret = SystemManagementModeCommunication::from(raw);
        assert_eq!(data, ret);
    }
}
