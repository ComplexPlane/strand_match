use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

use hex;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::{AsmFunction, Result, SDKLibrary, SMB2Rel};

struct XmlState {
    reader: Reader<BufReader<fs::File>>,
    buf: Vec<u8>,
}

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
    let mut state = XmlState {
        reader: Reader::from_file(xml_path)?,
        buf: Vec::new(),
    };
    let mut buf = Vec::new();

    loop {
        match state.reader.read_event(&mut buf) {
            Ok(Event::Start(e)) => match e.name() {
                b"PROGRAM" => {
                    let (lib_name, lib_base) = parse_program_name_and_base(&mut state, &e)?;
                    println!("{} {}", lib_name, lib_base);
                }
                _ => unimplemented!(),
            },

            Ok(Event::End(e)) => println!("Closing tag: {:#?}", e.name()),

            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(format!(
                    "Error at position {}: {:?}",
                    state.reader.buffer_position(),
                    e
                )
                .into())
            }
            _ => (), // There are several other `Event`s we do not consider here
        }
    }

    unimplemented!()
}

fn parse_program_name_and_base(state: &mut XmlState, e: &BytesStart) -> Result<(String, u32)> {
    let mut name = None;
    let mut base = None;

    for a in e.attributes() {
        let a = a?;
        match a.key {
            b"NAME" => name = Some(a.unescape_and_decode_value(&state.reader)?),
            b"IMAGE_BASE" => base = Some(a.unescape_and_decode_value(&state.reader)?),
            _ => (),
        }
    }

    // Validate that we received both a name and base String
    match (name, base) {
        (Some(mut name), Some(base)) => {
            // Remove .o extension
            name.truncate(name.len() - 2);
            let base = parse_hex_u32(&base)?;

            Ok((name, base))
        }

        _ => Err("Couldn't parse lib name".into()),
    }
}

fn parse_rel(xml_path: &Path, binary_path: &Path) -> Result<SMB2Rel> {
    unimplemented!()
}

fn parse_function_list(state: &mut XmlState) -> Result<Vec<AsmFunction>> {
    unimplemented!()
}

fn parse_function(state: &mut XmlState) -> Result<AsmFunction> {
    unimplemented!()
}

// Decode as big endian
fn parse_hex_u32(s: &str) -> Result<u32> {
    match hex::decode(s)?[..] {
        [b0, b1, b2, b3] => {
            Ok((b0 as u32) << 24 | (b1 as u32) << 16 | (b2 as u32) << 8 | (b3 as u32) << 0)
        }
        _ => Err(format!("Could not parse hexstring: {}", s).into()),
    }
}

fn parse_test(path: &str) -> Result<()> {
    let xml = r#"<tag1 att1 = "test", att2 = "poopes">
                    <tag2><!--Test comment-->Test</tag2>
                    <tag2>
                        Test 2
                    </tag2>
                </tag1>"#;

    let mut reader = Reader::from_file(&Path::new(path))?;
    reader.trim_text(true);

    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(e)) => {
                println!("Opening tag: {:#?}", e.name());
                println!("Opening tag decoded: {}", e.unescape_and_decode(&reader)?);
            }
            Ok(Event::End(e)) => println!("Closing tag: {:#?}", e.name()),

            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(
                    format!("Error at position {}: {:?}", reader.buffer_position(), e).into(),
                )
            }
            _ => (), // There are several other `Event`s we do not consider here
        }
    }

    Ok(())
}
