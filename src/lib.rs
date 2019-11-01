use failure;

mod parse;
mod util;
mod matching;
mod function;

type Result<T> = std::result::Result<T, failure::Error>;

pub fn run() -> Result<()> {
    let sdk_funcs = parse::parse_sdk_libs()?;
    let rel_funcs = parse::parse_rel()?;

    for func in &rel_funcs {
        println!("{}", func);
    }

//    matching::match_funcs(sdk_funcs, rel_funcs);

    Ok(())
}
