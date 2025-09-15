use acpi::error::Error;
use acpi::{MemoryMappedConfiguration, get};

fn main() -> Result<(), Error> {
    let mcfg = get::<MemoryMappedConfiguration>("MCFG")?;
    println!("{:?}", &mcfg);

    Ok(())
}
