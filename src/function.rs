use core::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fs::File;

use byteorder::{BigEndian, WriteBytesExt};

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

impl AsmFunction {
    fn hexstring_spaces(inst: u32) -> String {
        let nospaces = format!("{:08x}", inst);
        format!(
            "{} {} {} {}",
            &nospaces[0..2],
            &nospaces[2..4],
            &nospaces[4..6],
            &nospaces[6..8]
        )
    }

    #[allow(dead_code)]
    pub fn debug_export(&self) {
        println!("Exporting:\n{}", self);
        let mut file = File::create(format!("{}.bin", self.name)).unwrap();
        for instr in &self.code {
            file.write_u32::<BigEndian>(*instr).unwrap();
        }
    }

    pub fn full_name(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }
}

impl Display for AsmFunction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "AsmFunction:")?;
        writeln!(f, "    name: {}()", self.name)?;
        writeln!(f, "    namespace: {}", self.namespace)?;
        writeln!(f, "    library name: {}", self.lib_filename)?;
        writeln!(f, "    len: {}", self.len)?;
        writeln!(f, "    ghidra_addr: {:08x}", self.ghidra_addr)?;
        writeln!(f, "    code:")?;

        for (i, inst) in self.code.iter().enumerate() {
            write!(f, "        0x{:08x}  ", self.ghidra_addr + i as u32 * 4)?;
            if *inst == 0 {
                writeln!(f, "[ RELOCATED ]")?;
            } else {
                writeln!(f, "{}", AsmFunction::hexstring_spaces(*inst))?;
            }
        }

        Ok(())
    }
}

impl PartialOrd for AsmFunction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AsmFunction {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.namespace.cmp(&other.namespace) {
            Ordering::Equal => self.name.cmp(&other.name),
            ord => ord,
        }
    }
}

impl PartialEq for AsmFunction {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for AsmFunction {}
