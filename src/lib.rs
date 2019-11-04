use failure;

mod parse;
mod util;
mod matching;
mod function;
mod mapfile;

type Result<T> = std::result::Result<T, failure::Error>;

pub fn run() -> Result<()> {
    let sdk_funcs = parse::parse_dir("ghidra-export/sdk")?;
    let rel_funcs = parse::parse_dir("ghidra-export/smb1")?;

    matching::match_funcs(sdk_funcs, rel_funcs)?;

    Ok(())
}
