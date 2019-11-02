use std::fs::File;
use std::io::Write;

use crate::function::AsmFunction;
use crate::Result;

// Writes a symbol map which can be imported into Ghidra with the script
// Data->ImportSymbolsScript.py

pub fn export_mapfile(pairings: &[(&AsmFunction, &AsmFunction)]) -> Result<()> {
    let mut mapfile = File::create("smb2-symbols.map")?;

    for (sdk, rel) in pairings {
        writeln!(mapfile, "{} {} 0x{:08x}", sdk.name, sdk.namespace, rel.ghidra_addr)?;
    }
    Ok(())
}
