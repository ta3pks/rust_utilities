pub trait Base64EncodeExt {
    fn b64_enc(&self) -> String;
}
pub trait Base64DecodeExt {
    fn b64_dec(&self) -> Vec<u8>;
}
impl<T> Base64EncodeExt for T
where
    T: AsRef<[u8]>,
{
    fn b64_enc(&self) -> String {
        base64_light::base64_encode_bytes(self.as_ref())
    }
}

impl<T> Base64DecodeExt for T
where
    T: AsRef<str>,
{
    fn b64_dec(&self) -> Vec<u8> {
        base64_light::base64_decode(self.as_ref())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_base64() {
        let s = b"hello world";
        let e = s.b64_enc();
        assert_eq!(e, "aGVsbG8gd29ybGQ=");
        let d = e.b64_dec();
        assert_eq!(d, s);
    }
}
