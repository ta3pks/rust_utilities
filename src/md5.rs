use crate::{Base64EncodeExt, HexEncodeExt};

pub trait MD5Ext {
    fn md5(&self) -> [u8; 16];
    fn md5_hex(&self) -> String {
        self.md5().hex_lower()
    }
    #[cfg(feature = "base64")]
    fn md5_base64(&self) -> String {
        self.md5().b64_enc()
    }
}

impl<T> MD5Ext for T
where
    T: AsRef<[u8]>,
{
    fn md5(&self) -> [u8; 16] {
        md5::compute(self.as_ref()).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_md5() {
        let s = b"hello world";
        let e = s.md5_hex();
        assert_eq!(e, "5eb63bbbe01eeed093cb22bb8f5acdc3");
        #[cfg(feature = "base64")]
        assert_eq!(s.md5_base64(), "XrY7u+Ae7tCTyyK7j1rNww==");
    }
}
