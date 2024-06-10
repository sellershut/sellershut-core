use crate::user::UserType;

pub fn serialise_user_type<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::Error;

    let user_type = UserType::try_from(*value)
        .map_err(|_| S::Error::custom(format!("could not convert {value} to UserType enum")))?;

    serializer.serialize_str(user_type.as_str_name())
}

pub fn deserialise_user_type<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    // define a visitor that deserializes
    struct UserTypeVisitor;

    impl<'de> serde::de::Visitor<'de> for UserTypeVisitor {
        type Value = i32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a user type string Group/Individual")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match v {
                "Individual" => Ok(UserType::Individual as i32),
                "Group" => Ok(UserType::Group as i32),
                _ => Err(E::custom("unsupported user")),
            }
        }
    }

    deserializer.deserialize_any(UserTypeVisitor)
}
