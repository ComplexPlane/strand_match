use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, Seek, SeekFrom};

use hex;
use roxmltree::{Document, Node};
use regex::Regex;
use byteorder::{ReadBytesExt, BigEndian};

use crate::{Result, AsmFunction};

use lazy_static::lazy_static;

pub fn parse_sdk_libs() -> Result<Vec<AsmFunction>> {
    let mut result = Vec::new();

    let lib_dir = Path::new("ghidra-export/sdk");
    for entry in fs::read_dir(lib_dir)? {
        let entry_path = entry?.path();
        let xml_path = entry_path.with_extension("xml");
        let binary_path = entry_path.with_extension("bytes");

        let xml_str = fs::read_to_string(xml_path)?;
        let xml_tree = Document::parse(&xml_str)?;
        let root = xml_tree.root_element();

        let lib_name = root.attribute("NAME")
            .ok_or("Failed to get program name attribute")?;
        let base = root.attribute("IMAGE_BASE")
            .ok_or("Failed to get image base attribute")?;
        let base = parse_u32_hex(base)?;
        let namespace = parse_namespace(&root)?;

        parse_funcs(
            &binary_path,
            root,
            lib_name,
            namespace,
            base,
            &mut result,
        )?;
    }

    Ok(result)
}

fn parse_namespace<'a>(root: &'a Node) -> Result<&'a str> {
    let properties_node = root.children()
        .find(|c| c.has_tag_name("PROPERTIES"))
        .ok_or("Failed to find PROPERTIES node")?;

    let fsrl_property = properties_node.children()
        .find(|c| {
            match c.attribute("NAME") {
                Some(val) => val == "Program Information.FSRL",
                None => false,
            }
        }).ok_or("Failed to find FSRL property")?;

    let fsrl_val = fsrl_property.attribute("VALUE")
        .ok_or("Failed to get FSRL value")?;

    lazy_static!{
        static ref NAMESPACE_RE: Regex = Regex::new(r"/(\w+)\.((rel)|a)").unwrap();
    }

    match NAMESPACE_RE.captures(fsrl_val) {
        Some(caps) => Ok(caps.get(1).unwrap().as_str()),
        // TODO: why do I need to use .into() here but not with .ok_or()?
        None => Err("Failed to parse namespace".into()),
    }
}

fn parse_u32_hex(s: &str) -> Result<u32> {
    Ok(hex::decode(s)?.into_iter()
        .enumerate()
        .fold(0u32, |acc, (i, b)| {
            acc | (b as u32) << ((3 - i) as u32 * 8)
        })
    )
}

fn parse_funcs(
    binary_path: &Path,
    xml_root: Node,

    lib_filename: &str,
    namespace: &str,
    ghidra_base: u32,

    funcs: &mut Vec<AsmFunction>) -> Result<()> {

    let f = File::open(binary_path)?;
    let mut f = BufReader::new(f);

    let func_list_elem = xml_root.children()
        .find(|c| c.has_tag_name("FUNCTIONS"))
        .ok_or("Couldn't find FUNCTIONS element")?;

    for func_elem in func_list_elem.children() {
        if func_elem.tag_name().name() != "FUNCTION" {
            // Apparently, whitespace strings count as child elements...
            continue;
        }

        let name = func_elem.attribute("NAME")
            .ok_or("Failed to get function name")?;

        let addr_range = func_elem.children()
            .find(|c| c.has_tag_name("ADDRESS_RANGE"))
            .ok_or("Failed to get function address range")?;
        let start = addr_range.attribute("START")
            .ok_or("Failed to get function address range start")?;
        let end = addr_range.attribute("END")
            .ok_or("Failed to get function address range start")?;

        let start = parse_u32_hex(start)?;
        let end = parse_u32_hex(end)?; // Location of last byte, inclusive

        if start == end {
            // Thunk function, ignore
            continue;
        }

        // Read function's code
        let func_len = end - start + 1;
        let mut code = vec![0; (func_len / 4) as usize];
        f.seek(SeekFrom::Start((start - ghidra_base) as u64))?;
        f.read_u32_into::<BigEndian>(&mut code)?;

        funcs.push(AsmFunction {
            lib_filename: String::from(lib_filename),
            namespace: String::from(namespace),
            ghidra_base,

            name: String::from(name),
            ghidra_addr: start,
            len: func_len,
            code,
        });
    }

    Ok(())
}
