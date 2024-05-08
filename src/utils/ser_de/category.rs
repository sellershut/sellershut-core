#[cfg(feature = "surrealdb")]
pub fn serialize_string<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use crate::{utils::ser_de::serialise_thing, Table};

    serialise_thing(Table::Category, value, serializer)
}

#[cfg(feature = "surrealdb")]
pub fn serialize_strings<S>(value: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use crate::{utils::ser_de::serialise_things, Table};

    serialise_things(Table::Category, value, serializer)
}

#[cfg(feature = "surrealdb")]
pub fn serialize_optional_string<S>(
    value: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use crate::{utils::ser_de::serialise_optional_thing, Table};

    serialise_optional_thing(Table::Category, value, serializer)
}
