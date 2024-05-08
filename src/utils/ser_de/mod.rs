#[cfg(feature = "categories")]
pub mod category;

#[cfg(feature = "users")]
pub mod user;

#[cfg(feature = "users")]
pub mod account;

#[cfg(all(feature = "surrealdb", any(feature = "categories", feature = "users")))]
fn id_to_string(id: &surrealdb_core::sql::Id) -> String {
    let id = id.to_raw();
    id.split(':')
        .next()
        .unwrap_or(&id)
        .chars()
        .filter(|&c| c != '⟨' && c != '⟩')
        .collect()
}

#[cfg(all(feature = "surrealdb", any(feature = "categories", feature = "users")))]
fn serialise_thing<S>(table: crate::Table, value: &str, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::Serialize;

    let thing = surrealdb_core::sql::Thing::from((table.to_string().as_str(), value));
    thing.serialize(s)
}

#[cfg(all(feature = "surrealdb", feature = "categories"))]
fn serialise_things<S>(table: crate::Table, value: &[String], s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::Serialize;
    let thing: Vec<_> = value
        .iter()
        .map(|value| surrealdb_core::sql::Thing::from((table.to_string().as_str(), value.as_str())))
        .collect();
    thing.serialize(s)
}

#[cfg(all(feature = "surrealdb", feature = "categories"))]
fn serialise_optional_thing<S>(
    table: crate::Table,
    value: &Option<String>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::Serialize;
    let thing: Option<_> = value.clone().map(|value| {
        surrealdb_core::sql::Thing::from((table.to_string().as_str(), value.as_str()))
    });
    thing.serialize(s)
}

#[cfg(all(feature = "surrealdb", any(feature = "categories", feature = "users")))]
pub fn deserialize_surreal_thing<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::Deserialize;

    // define a visitor that deserializes
    struct SurrealThingVisitor;

    impl<'de> serde::de::Visitor<'de> for SurrealThingVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a surrealdb record id")
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let record_id: surrealdb_core::sql::Thing =
                Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;

            Ok(id_to_string(&record_id.id))
        }
    }

    deserializer.deserialize_any(SurrealThingVisitor)
}

#[cfg(all(feature = "surrealdb", feature = "categories"))]
pub fn deserialize_optional_surreal_thing<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    // define a visitor that deserializes
    struct SurrealThingVisitor;

    impl<'de> serde::de::Visitor<'de> for SurrealThingVisitor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an optional surrealdb record id")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error, {
            Ok(None)
        }

        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let record_id: surrealdb_core::sql::Thing =
                Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))?;

            Ok(Some(id_to_string(&record_id.id)))
        }
    }

    d.deserialize_any(SurrealThingVisitor)
}

#[cfg(all(feature = "surrealdb", feature = "categories"))]
pub fn deserialize_surreal_things<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    // define a visitor that deserializes
    struct SurrealThingVisitor;

    impl<'de> serde::de::Visitor<'de> for SurrealThingVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a list containing a surrealdb things")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut vec: Vec<String>;

            if let Some(size) = seq.size_hint() {
                vec = Vec::with_capacity(size);
            } else {
                vec = Vec::new();
            }

            while let Some(thing) = seq.next_element::<surrealdb_core::sql::Thing>()? {
                vec.push(id_to_string(&thing.id));
            }

            Ok(vec)
        }
    }

    deserializer.deserialize_any(SurrealThingVisitor)
}
