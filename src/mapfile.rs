use std::fs::File;
use std::io::Write;

use crate::function::AsmFunction;

// Writes a Dolphin-compatible symbol map which can be imported into Ghidra with the script

pub fn export_mapfile(pairings: &[(&AsmFunction, &AsmFunction)]) -> Result<(), anyhow::Error> {
    let mut mapfile = File::create("bb-symbols.map")?;

    writeln!(mapfile, ".text section layout")?;
    for (sdk, rel) in pairings {
        writeln!(
            mapfile,
            "{:08x} {:08x} {:08x} 0 {}",
            rel.ghidra_addr,
            sdk.len,
            rel.ghidra_addr,
            sdk.full_name()
        )?;
    }
    writeln!(mapfile, "\n.data section layout")?;

    Ok(())
}
