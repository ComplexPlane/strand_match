use std::fs::File;

use byteorder::{WriteBytesExt, BigEndian};

use crate::Result;
use crate::AsmFunction;

pub fn parse_u32_hex(s: &str) -> Result<u32> {
    let s = if s.len() >= 2 && &s[0..2].to_lowercase() == "0x" {
        &s[2..]
    } else {
        s
    };

    Ok(u32::from_str_radix(s, 16)?)
}

pub fn export_function(func: &AsmFunction) {
    let mut file = File::create(format!("{}.bin", func.name)).unwrap();
    for instr in &func.code {
        file.write_u32::<BigEndian>(*instr).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hexstring() {
        assert_eq!(parse_u32_hex("0x52ac0"), Some(338624));
        assert_eq!(parse_u32_hex("0X52ac0"), Some(338624));
        assert_eq!(parse_u32_hex("42"), Some(66));
    }
}
