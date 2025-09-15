use acpi::error::Error;
use acpi::{BootGraphicsResource, get};

fn main() -> Result<(), Error> {
    let bgrt = get::<BootGraphicsResource>("BGRT")?;
    println!("{:?}", &bgrt);

    Ok(())
}
