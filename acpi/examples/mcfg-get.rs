use acpi::error::Error;
use acpi::{get, MemoryMappedConfiguration};

fn main() -> Result<(), Error> {
    let mcfg = get::<MemoryMappedConfiguration>("MCFG")?;
    println!("{:?}", &mcfg);

    Ok(())
}
