#[cfg(all(
    feature = "surrealdb",
    feature = "serde",
    any(feature = "categories", feature = "users")
))]
pub fn get_string_from_thing<E>(v: &str) -> Result<String, E>
where
    E: serde::de::Error,
{
    use std::str::FromStr;
    let thing = surrealdb_core::sql::Thing::from_str(v)
        .map_err(|_| E::custom("could not map thing to string"))?;

    let id_to_string = |id: &surrealdb_core::sql::Id| -> String {
        let id = id.to_raw();
        id.split(':')
            .next()
            .unwrap_or(&id)
            .chars()
            .filter(|&c| c != '⟨' && c != '⟩')
            .collect()
    };

    Ok(id_to_string(&thing.id))
}

#[cfg(all(
    feature = "surrealdb",
    feature = "serde",
    any(feature = "categories", feature = "users")
))]
pub fn deserialize_surreal_thing<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    // define a visitor that deserializes
    struct SurrealThingVisitor;

    impl<'de> serde::de::Visitor<'de> for SurrealThingVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing a surrealdb thing")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            get_string_from_thing(v)
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            get_string_from_thing(&v)
        }
    }

    deserializer.deserialize_any(SurrealThingVisitor)
}

#[cfg(all(feature = "surrealdb", feature = "serde", feature = "categories"))]
pub fn deserialize_optional_surreal_thing<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let res =
        serde::Deserialize::deserialize(d).map(|x: Option<_>| x.map(get_string_from_thing))?;
    match res {
        Some(data) => {
            let data = data?;
            Ok(Some(data))
        }
        None => Ok(None),
    }
}

#[cfg(all(feature = "surrealdb", feature = "serde", feature = "categories"))]
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

            while let Some(item) = seq.next_element::<&str>()? {
                vec.push(get_string_from_thing(item)?)
            }

            Ok(vec)
        }
    }

    deserializer.deserialize_any(SurrealThingVisitor)
}
