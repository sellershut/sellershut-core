#![allow(clippy::vec_init_then_push)]
#![allow(unused_mut)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/proto");

    let mut protos: Vec<&str> = Vec::new();

    dbg!("created vec");

    #[cfg(any(feature = "categories", feature = "users"))]
    protos.push("src/proto/common.proto");

    #[cfg(feature = "categories")]
    protos.push("src/proto/category.proto");

    #[cfg(feature = "users")]
    {
        protos.push("src/proto/user.proto");
        protos.push("src/proto/session.proto");
        protos.push("src/proto/account_provider.proto");
        protos.push("src/proto/account.proto");
    }

    if !protos.is_empty() {
        #[cfg(feature = "tonic")]
        build(&protos)?;
    }
    Ok(())
}

#[cfg(feature = "tonic")]
fn build(protos: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    use std::{env, path::PathBuf};

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    for feature in protos {
        let mut config = tonic_build::configure()
            .build_server(cfg!(feature = "tonic-rpc")) // for traits in services
            .build_client(cfg!(feature = "tonic-rpc"))
            .build_transport(cfg!(feature = "tonic-rpc"));

        #[cfg(all(feature = "serde", any(feature = "categories", feature = "users")))]
        let serde_type_attrs = serde_type_attrs();

        #[cfg(all(feature = "serde", any(feature = "categories", feature = "users")))]
        let mut config = config.type_attribute(serde_type_attrs.0, serde_type_attrs.1);

        let mut config = config.type_attribute(".", "#[cfg_attr(test, derive(fake::Dummy))]");

        #[cfg(all(feature = "serde", feature = "surrealdb"))]
        let config = {
            let mut ids: Vec<&str> = Vec::new();

            #[cfg(feature = "users")]
            {
                ids.push(".user.User.id");
                ids.push(".oauth.session.OauthSession.user_id");
                ids.push(".oauth.account.OauthAccount.user_id");
                ids.push(".oauth.account.OauthAccount.account_provider_id");
                ids.push(".oauth.session.OauthSession.account_provider_id");
                ids.push(".oauth.account_provider.OauthProvider.id");
            }

            #[cfg(feature = "categories")]
            ids.push(".category.Category.id");

            for field in ids {
                config = config.field_attribute(
                field,
                "#[serde(deserialize_with = \"crate::utils::ser_de::deserialize_surreal_thing\")]",
            );
            }

            let mut id_list: Vec<&str> = Vec::new();

            #[cfg(feature = "categories")]
            id_list.push(".category.Category.sub_categories");

            for field in id_list {
                config = config.field_attribute(
                field,
                "#[serde(deserialize_with = \"crate::utils::ser_de::deserialize_surreal_things\")]",
            );
            }

            let mut optional_ids: Vec<&str> = Vec::new();
            optional_ids.push(".category.Category.parent_id");

            for field in optional_ids {
                config = config.field_attribute(
                field,
                "#[serde(deserialize_with = \"crate::utils::ser_de::deserialize_optional_surreal_thing\")]",
            );
            }

            // serialize
            #[cfg(feature = "categories")]
            {
                config = config.field_attribute(
                ".category.Category.id",
                "#[serde(serialize_with = \"crate::utils::ser_de::category::serialize_string\")]",
                ).field_attribute(".category.Category.sub_categories",
                    "#[serde(serialize_with = \"crate::utils::ser_de::category::serialize_strings\")]",
                ).field_attribute(".category.Category.parent_id",
                    "#[serde(serialize_with = \"crate::utils::ser_de::category::serialize_optional_string\")]",
                );
            }

            #[cfg(feature = "users")]
            {
                config = config
                .field_attribute(
                    ".user.User.id",
                    "#[serde(serialize_with = \"crate::utils::ser_de::user::serialize_string\")]",
                )
                .field_attribute(
                    ".oauth.session.OauthSession.user_id",
                    "#[serde(serialize_with = \"crate::utils::ser_de::user::serialize_string\")] #[serde(rename =\"in\")]",
                )
                .field_attribute(
                    ".oauth.account.OauthAccount.user_id",
                    "#[serde(serialize_with = \"crate::utils::ser_de::user::serialize_string\")] #[serde(rename = \"in\")]",
                )
                .field_attribute(
                    ".oauth.account.OauthAccount.account_provider_id",
                    "#[serde(serialize_with = \"crate::utils::ser_de::account::serialize_string\")] #[serde(rename = \"out\")]",
                )
                .field_attribute(
                    ".oauth.session.OauthSession.account_provider_id",
                    "#[serde(serialize_with = \"crate::utils::ser_de::account::serialize_string\")] #[serde(rename = \"out\")]",
                )
                .field_attribute(
                    ".oauth.account_provider.OauthProvider.id",
                    "#[serde(serialize_with = \"crate::utils::ser_de::account::serialize_string\")]",
                );
            }

            config
        };

        println!("feature: {feature:?}");
        let name = feature
            .split('/')
            .last()
            .and_then(|last| last.split('.').next())
            .expect("valid proto paths");

        config
            .file_descriptor_set_path(out_dir.join(format!("{name}_descriptor.bin")))
            .compile(&[feature], &["src"])?;
    }

    Ok(())
}

#[cfg(all(feature = "serde", any(feature = "categories", feature = "users")))]
fn serde_type_attrs() -> (impl AsRef<str>, impl AsRef<str>) {
    (
        ".",
        "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all = \"snake_case\")]",
    )
}
