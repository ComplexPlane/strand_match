use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

use hex;
use roxmltree;

use crate::{AsmFunction, Result, SDKLibrary, SMB2Rel};


pub fn parse() {
    //    parse_library_dir();
    let xml_path = Path::new("ghidra-export/sdk/00ac7a7bcfcaa2eb1471774939a39c60.xml");
    let bin_path = Path::new("ghidra-export/sdk/00ac7a7bcfcaa2eb1471774939a39c60.bytes");
    parse_library(xml_path, bin_path);
}

fn parse_library_dir() -> Result<Vec<SDKLibrary>> {
    let lib_dir = Path::new("ghidra-export/sdk");
    for entry in fs::read_dir(lib_dir)? {
        let entry_path = entry?.path();
        let xml_path = entry_path.with_extension("xml");
        let binary_path = entry_path.with_extension("bytes");
        parse_library(&xml_path, &binary_path);
    }
    unimplemented!()
}

fn parse_library(xml_path: &Path, binary_path: &Path) -> Result<SDKLibrary> {
    let xml_str = fs::read_to_string(xml_path)?;
    let parsed = roxmltree::Document::parse(&xml_str)?;
    println!("name: {}", parsed.root().attribute("NAME").expect("TODO"));
    unimplemented!()
}

//fn parse_program_name_and_base(state: &mut XmlState, e: &BytesStart) -> Result<(String, u32)> {
//}
//
//fn parse_rel(xml_path: &Path, binary_path: &Path) -> Result<SMB2Rel> {
//    unimplemented!()
//}
//
//fn parse_function_list(state: &mut XmlState) -> Result<Vec<AsmFunction>> {
//    unimplemented!()
//}
//
//fn parse_function(state: &mut XmlState) -> Result<AsmFunction> {
//    unimplemented!()
//}
//
//// Decode as big endian
//fn parse_hex_u32(s: &str) -> Result<u32> {
//    match hex::decode(s)?[..] {
//        [b0, b1, b2, b3] => {
//            Ok((b0 as u32) << 24 | (b1 as u32) << 16 | (b2 as u32) << 8 | (b3 as u32) << 0)
//        }
//        _ => Err(format!("Could not parse hexstring: {}", s).into()),
//    }
//}
