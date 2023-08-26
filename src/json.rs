pub trait JsonEncodeExt
where
    Self: serde::Serialize,
{
    fn to_json(&self) -> Result<serde_json::Value, serde_json::Error>;
    fn to_json_string(&self) -> Result<String, serde_json::Error>;
    fn to_json_pretty_string(&self) -> Result<String, serde_json::Error>;
}
pub trait JsonDecodeExt {
    fn parse_json<T>(&self) -> Result<T, serde_json::Error>
    where
        T: serde::de::DeserializeOwned,
        Self: AsRef<[u8]>;
}
impl<T> JsonEncodeExt for T
where
    T: serde::Serialize,
{
    fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
    fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    fn to_json_pretty_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
impl<T> JsonDecodeExt for T
where
    T: AsRef<[u8]>,
{
    fn parse_json<U>(&self) -> Result<U, serde_json::Error>
    where
        U: serde::de::DeserializeOwned,
    {
        serde_json::from_slice(self.as_ref())
    }
}
