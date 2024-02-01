use acpi::error::Error;
use acpi::{get_raw_table, table_types};

fn main() -> Result<(), Error> {
    let sigs = table_types()?;
    for sig in sigs {
        let table = get_raw_table(&sig)?;
        println!("{}", &table.signature);
    }

    Ok(())
}
