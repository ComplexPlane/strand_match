use failure;

mod parse;
mod util;

type Result<T> = std::result::Result<T, failure::Error>;

#[derive(Debug)]
pub struct AsmFunction {
    // Metadata
    // Could be shared between certain functions, but cannot return metadata and functions in same
    // struct. Need either reference counting, or to compute metadata separately from functions.
    // Also, it's more ergonomic to associate the metadata with the functions
    pub lib_filename: String,
    pub namespace: String,

    pub name: String,
    pub ghidra_addr: u32,
    pub len: u32,
    pub code: Vec<u32>,
}

pub fn run() -> Result<()> {
    let sdk_funcs = parse::parse_sdk_libs()?;
    let rel_funcs = parse::parse_rel()?;

    let sdk_idx = 2573;
    let rel_idx = 6320;

    println!("sdk function {}:\n{:#?}", sdk_idx, sdk_funcs[sdk_idx]);
    println!("rel function {}:\n{:#?}", rel_idx, rel_funcs[rel_idx]);

    util::export_function(&rel_funcs[rel_idx]);
    util::export_function(&sdk_funcs[sdk_idx]);

    Ok(())
}
