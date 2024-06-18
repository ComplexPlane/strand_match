pub fn parse_u32_hex(s: &str) -> anyhow::Result<u32> {
    let s = if s.len() >= 2 && &s[0..2].to_lowercase() == "0x" {
        &s[2..]
    } else {
        s
    };

    Ok(u32::from_str_radix(s, 16)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hexstring() {
        assert_eq!(parse_u32_hex("0x52ac0").ok(), Some(338624));
        assert_eq!(parse_u32_hex("0X52ac0").ok(), Some(338624));
        assert_eq!(parse_u32_hex("42").ok(), Some(66));
        assert_eq!(parse_u32_hex("poopoo").ok(), None);
    }
}
