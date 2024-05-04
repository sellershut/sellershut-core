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
    let mut config = tonic_build::configure()
        .build_server(cfg!(feature = "tonic-rpc")) // for traits in services
        .build_client(cfg!(feature = "tonic-rpc"))
        .build_transport(cfg!(feature = "tonic-rpc"));

    #[cfg(all(feature = "serde", any(feature = "categories", feature = "users")))]
    let serde_type_attrs = serde_type_attrs();

    #[cfg(all(feature = "serde", any(feature = "categories", feature = "users")))]
    let mut config = config.type_attribute(serde_type_attrs.0, serde_type_attrs.1);

    #[cfg(all(feature = "serde", feature = "surrealdb"))]
    let config = {
        let mut ids: Vec<&str> = Vec::new();

        #[cfg(feature = "users")]
        ids.push(".user.User.id");

        #[cfg(feature = "categories")]
        ids.push(".category.Category.id");

        for field in ids {
            config = config.field_attribute(
                field,
                "#[serde(deserialize_with = \"crate::utils::deserialize_surreal_thing\")]",
            );
        }

        let mut id_list: Vec<&str> = Vec::new();

        #[cfg(feature = "categories")]
        id_list.push(".category.Category.sub_categories");

        for field in id_list {
            config = config.field_attribute(
                field,
                "#[serde(deserialize_with = \"crate::utils::deserialize_surreal_things\")]",
            );
        }

        let mut optional_ids: Vec<&str> = Vec::new();
        optional_ids.push(".category.Category.parent_id");

        for field in optional_ids {
            config = config.field_attribute(
                field,
                "#[serde(deserialize_with = \"crate::utils::deserialize_optional_surreal_thing\")]",
            );
        }

        config
    };

    config.compile(protos, &["src"])?;

    Ok(())
}

#[cfg(all(feature = "serde", any(feature = "categories", feature = "users")))]
fn serde_type_attrs() -> (impl AsRef<str>, impl AsRef<str>) {
    (
        ".",
        "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all = \"snake_case\")]",
    )
}
