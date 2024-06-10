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

        #[cfg(feature = "sqlx")]
        {
            if cfg!(feature = "categories") {
                config = config.type_attribute(".category.Category", "#[derive(sqlx::FromRow)]");
            }
        }

        #[cfg(all(feature = "serde", any(feature = "categories", feature = "users")))]
        let serde_type_attrs = serde_type_attrs();

        #[cfg(all(feature = "serde", any(feature = "categories", feature = "users")))]
        let config = config.type_attribute(serde_type_attrs.0, serde_type_attrs.1);

        let mut config = config.type_attribute(".", "#[cfg_attr(test, derive(fake::Dummy))]");

        config = config.field_attribute(
            ".user.User.user_type",
            "#[cfg_attr(test, dummy(faker = \"0..1\"))]",
        );

        #[cfg(all(feature = "users", feature = "serde"))]
        {
            config = config
                .field_attribute(
                    ".user.User.user_type",
                    "#[serde(serialize_with = \"crate::utils::ser_de::user::serialise_user_type\")]",
                )
                .field_attribute(
                    ".user.User.user_type",
                    "#[serde(deserialize_with = \"crate::utils::ser_de::user::deserialise_user_type\")]",
                );
        }
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
