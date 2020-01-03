use failure;

mod function;
mod mapfile;
mod matching;
mod parse;
mod util;

type Result<T> = std::result::Result<T, failure::Error>;

pub fn run() -> Result<()> {
    let sdk_funcs = parse::parse_dir("ghidra-export/wii-sdk-2006")?;
    let rel_funcs = parse::parse_dir("ghidra-export/bb")?;

    matching::match_funcs(sdk_funcs, rel_funcs)?;

    Ok(())
}
