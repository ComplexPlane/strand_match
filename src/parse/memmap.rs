use crate::util;
use crate::Result;

use failure::{bail, format_err};
use roxmltree::Node;

// Maybe would work better with Vec<(u32, u32)> or something?
// Not sure how I'd call binary search then
pub struct MemoryMap {
    pub ghidra_addrs: Vec<u32>,
    pub file_offsets: Vec<u32>,
}

impl MemoryMap {
    pub fn parse(root: Node) -> Result<MemoryMap> {
        let memory_map_elem = root
            .children()
            .find(|c| c.has_tag_name("MEMORY_MAP"))
            .ok_or(format_err!("Could not find MEMORY_MAP element"))?;

        let mut memmap = MemoryMap {
            ghidra_addrs: Vec::new(),
            file_offsets: Vec::new(),
        };

        for section_elem in memory_map_elem.children() {
            if !section_elem.has_tag_name("MEMORY_SECTION") {
                continue;
            }

            // Only parse executable sections
            let executable = section_elem
                .attribute("PERMISSIONS")
                .ok_or(format_err!("Failed to parse section permissions"))?
                .contains('x');
            if !executable {
                continue;
            }

            // Get start address of section in ghidra
            let ghidra_addr = section_elem
                .attribute("START_ADDR")
                .ok_or(format_err!("Failed to parse section start address"))?;
            let ghidra_addr = util::parse_u32_hex(ghidra_addr)?;

            // Get section start offset from start of file
            let mem_contents_elem = section_elem
                .children()
                .find(|c| c.has_tag_name("MEMORY_CONTENTS"));
            let mem_contents_elem = match mem_contents_elem {
                None => continue, // Memory-only section, not a code section for sure
                Some(e) => e,
            };
            let file_offset = mem_contents_elem
                .attribute("FILE_OFFSET")
                .ok_or(format_err!("Could not parse FILE_OFFSET"))?;
            let file_offset = util::parse_u32_hex(file_offset)?;

            memmap.ghidra_addrs.push(ghidra_addr);
            memmap.file_offsets.push(file_offset);
        }

        Ok(memmap)
    }

    pub fn find_segment_idx(&self, ghidra_addr: u32) -> Result<usize> {
        match self.ghidra_addrs.binary_search(&ghidra_addr) {
            Ok(i) => Ok(i),
            Err(i) => {
                if i == 0 {
                    bail!("Ghidra addr not in any memory segment")
                } else {
                    Ok(i - 1)
                }
            }
        }
    }
}
