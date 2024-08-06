use secrecy::Secret;
use serde::Serializer;

pub fn stars<S>(_secret: &Secret<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("*********")
}
