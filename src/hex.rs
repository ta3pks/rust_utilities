pub trait HexEncodeExt {
    fn hex_lower(&self) -> String;
    fn hex_upper(&self) -> String {
        self.hex_lower().to_uppercase()
    }
}
pub trait HexDecodeExt {
    fn hex_dec(&self) -> Vec<u8>;
}

impl<T> HexEncodeExt for T
where
    T: AsRef<[u8]>,
{
    fn hex_lower(&self) -> String {
        self.as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
}

impl<T> HexDecodeExt for T
where
    T: AsRef<str>,
{
    fn hex_dec(&self) -> Vec<u8> {
        let s = self.as_ref();
        let mut v = Vec::with_capacity(s.len() / 2);
        for i in 0..s.len() / 2 {
            let b = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap();
            v.push(b);
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hex() {
        let s = b"hello world";
        let e = s.hex_lower();
        assert_eq!(e, "68656c6c6f20776f726c64");
        assert_eq!(s.hex_upper(), "68656C6C6F20776F726C64");
        let d = e.hex_dec();
        assert_eq!(d, s);
    }
}
