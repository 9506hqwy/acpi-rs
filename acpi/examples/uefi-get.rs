use acpi::error::Error;
use acpi::{SystemManagementModeCommunication, get};

fn main() -> Result<(), Error> {
    let uefi = get::<SystemManagementModeCommunication>("UEFI")?;
    println!("{:?}", &uefi);

    Ok(())
}
