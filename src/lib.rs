mod parse;

struct SDKLibrary {
    lib_filename: String,
    archive_filename: String,
    ghidra_binary_filename: String,
    ghidra_base: u32,
    functions: Vec<AsmFunction>,
}

struct SMB2Rel {
    ghidra_base: u32,
    functions: Vec<AsmFunction>,
}

struct AsmFunction {
    name: String,
    ghidra_addr: u32,
    len: u32,
    code: Vec<u32>,
}

/*
TODO:
 * Walk directories for XML / memory map files
 * Parse library, rel, and function structs
 * Match functions between libraries and rel
*/

// Similar to the way the ripgrep frontend handles errors
// Maybe better way to do this in the future?
// Reading rust book chapter on Box stuff might help
// Also the `failure` crate
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn run() -> Result<()> {
    parse::parse();
    Ok(())
}
