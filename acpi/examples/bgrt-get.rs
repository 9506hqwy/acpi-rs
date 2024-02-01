use acpi::error::Error;
use acpi::{get, BootGraphicsResource};

fn main() -> Result<(), Error> {
    let bgrt = get::<BootGraphicsResource>("BGRT")?;
    println!("{:?}", &bgrt);

    Ok(())
}
