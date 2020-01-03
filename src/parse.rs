use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::path::Path;

use byteorder::{BigEndian, ReadBytesExt};
use failure::{bail, format_err};
use lazy_static::lazy_static;
use regex::Regex;
use roxmltree::{Document, Node};

use crate::function::AsmFunction;
use crate::util;
use crate::Result;

use crate::util::parse_u32_hex;
use memmap::MemoryMap;

mod memmap;

pub fn parse_dir(path: &str) -> Result<Vec<AsmFunction>> {
    let mut funcs = Vec::new();

    let lib_dir = Path::new(path);
    for entry in fs::read_dir(lib_dir)? {
        let path = entry?.path();
        let ext = path
            .extension()
            .ok_or(format_err!("File with no extension?"))?;
        if ext == "xml" {
            parse_module(&path, &mut funcs)?;
        }
    }

    Ok(funcs)
}

fn parse_module(xml_path: &Path, funcs: &mut Vec<AsmFunction>) -> Result<()> {
    let binary_path = xml_path.with_extension("bytes");

    let xml_str = fs::read_to_string(xml_path)?;
    let xml_tree = Document::parse(&xml_str)?;
    let root = xml_tree.root_element();

    let lib_name = root
        .attribute("NAME")
        .ok_or(format_err!("Failed to get program name attribute"))?;
    let namespace = parse_namespace(&root)?;

    parse_funcs(&binary_path, root, lib_name, namespace, funcs)
}

fn parse_namespace<'a>(root: &'a Node) -> Result<&'a str> {
    let properties_node = root
        .children()
        .find(|c| c.has_tag_name("PROPERTIES"))
        .ok_or(format_err!("Failed to find PROPERTIES node"))?;

    let fsrl_property = properties_node
        .children()
        .find(|c| match c.attribute("NAME") {
            Some(val) => val == "Program Information.FSRL",
            None => false,
        })
        .ok_or(format_err!("Failed to find FSRL property"))?;

    let fsrl_val = fsrl_property
        .attribute("VALUE")
        .ok_or(format_err!("Failed to get FSRL value"))?;

    lazy_static! {
        static ref NAMESPACE_RE: Regex = Regex::new(r"/([\w\.]+)\.((rel)|(dol)|a)").unwrap();
    }

    match NAMESPACE_RE.captures(fsrl_val) {
        Some(caps) => Ok(caps.get(1).unwrap().as_str()),
        None => bail!("Failed to parse namespace"),
    }
}

// Returns a list of instruction addresses which have been relocated.
// NOTE: does not return the addresses of the changed bytes inside the instructions themselves
fn parse_relocation_table(root: Node) -> Result<Vec<u32>> {
    let reloc_table = root
        .children()
        .find(|c| c.has_tag_name("RELOCATION_TABLE"))
        .ok_or(format_err!("RELOCATION_TABLE not found"))?;

    let mut reloc_vec = Vec::new();

    for reloc in reloc_table.children() {
        if !reloc.has_tag_name("RELOCATION") {
            continue;
        }

        let addr = reloc
            .attribute("ADDRESS")
            .ok_or(format_err!("ADDRESS attribute not found"))?;
        let addr = parse_u32_hex(addr)?;
        let addr = addr - (addr % 4); // Get address of instruction itself
        reloc_vec.push(addr);
    }

    Ok(reloc_vec)
}

fn parse_funcs(
    binary_path: &Path,
    root: Node,

    lib_filename: &str,
    namespace: &str,

    funcs: &mut Vec<AsmFunction>,
) -> Result<()> {
    let memory_map = MemoryMap::parse(root)?;

    let f = File::open(binary_path)?;
    let mut f = BufReader::new(f);
    let relocs: HashSet<_> = parse_relocation_table(root)?.into_iter().collect();

    let func_list_elem = root
        .children()
        .find(|c| c.has_tag_name("FUNCTIONS"))
        .ok_or(format_err!("Couldn't find FUNCTIONS element"))?;

    for func_elem in func_list_elem.children() {
        if !func_elem.has_tag_name("FUNCTION") {
            // Apparently, whitespace strings count as child elements...
            continue;
        }

        let name = func_elem
            .attribute("NAME")
            .ok_or(format_err!("Failed to get function name"))?;

        let addr_range = func_elem
            .children()
            .find(|c| c.has_tag_name("ADDRESS_RANGE"))
            .ok_or(format_err!("Failed to get function address range"))?;
        let start = addr_range
            .attribute("START")
            .ok_or(format_err!("Failed to get function address range start"))?;
        let end = addr_range
            .attribute("END")
            .ok_or(format_err!("Failed to get function address range start"))?;

        let start = util::parse_u32_hex(start)?;
        let end = util::parse_u32_hex(end)?; // Location of last byte, inclusive

        if start == end {
            // Thunk function, ignore
            continue;
        }

        // Compute file offset of function
        let seg = memory_map.find_segment_idx(start)?;
        let func_file_pos = start - memory_map.ghidra_addrs[seg] + memory_map.file_offsets[seg];

        // Read function's code
        let func_len = end - start + 1;
        let mut code = vec![0; (func_len / 4) as usize];
        f.seek(SeekFrom::Start(func_file_pos as u64))?;
        f.read_u32_into::<BigEndian>(&mut code)?;

        // Flag instructions which are relocated so they can be not counted in later comparisons
        for (i, inst) in code.iter_mut().enumerate() {
            if relocs.contains(&(start + i as u32 * 4)) {
                *inst = 0;
            }
        }

        funcs.push(AsmFunction {
            lib_filename: String::from(lib_filename),
            namespace: String::from(namespace),

            name: String::from(name),
            ghidra_addr: start,
            len: func_len,
            code,
        });
    }

    Ok(())
}
