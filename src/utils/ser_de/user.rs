#[cfg(feature = "surrealdb")]
pub fn serialize_string<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use crate::{utils::ser_de::serialise_thing, Table};

    serialise_thing(Table::User, value, serializer)
}
