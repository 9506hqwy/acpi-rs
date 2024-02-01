use acpi::error::Error;
use acpi::{get, SystemManagementModeCommunication};

fn main() -> Result<(), Error> {
    let uefi = get::<SystemManagementModeCommunication>("UEFI")?;
    println!("{:?}", &uefi);

    Ok(())
}
