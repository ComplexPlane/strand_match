mod function;
mod mapfile;
mod matching;
mod parse;
mod util;

pub fn run() -> Result<(), anyhow::Error> {
    let sdk_funcs = parse::parse_dir("ghidra-export/wii-sdk-2006")?;
    let rel_funcs = parse::parse_dir("ghidra-export/bb")?;

    matching::match_funcs(sdk_funcs, rel_funcs)?;

    Ok(())
}
