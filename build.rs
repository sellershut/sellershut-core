fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/proto");

    #[allow(unused_mut)]
    let mut protos: Vec<&str> = Vec::new();

    dbg!("created vec");

    #[cfg(any(feature = "categories", feature = "users"))]
    protos.push("src/proto/common.proto");

    #[cfg(feature = "categories")]
    protos.push("src/proto/category.proto");

    #[cfg(feature = "users")]
    protos.push("src/proto/user.proto");

    if !protos.is_empty() {
        #[cfg(feature = "tonic")]
        build(&protos)?;
    }
    Ok(())
}

#[cfg(feature = "tonic")]
fn build(protos: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    for f in protos {
        let config = tonic_build::configure()
            .build_server(
                if f.ends_with("category.proto")
                    && cfg!(all(feature = "categories", feature = "tonic-rpc"))
                {
                    true
                } else {
                    f.ends_with("users.proto")
                        && cfg!(all(feature = "users", feature = "tonic-rpc"))
                },
            ) // for traits in services
            .build_client(cfg!(feature = "tonic-rpc"))
            .build_transport(cfg!(feature = "tonic-rpc"));

        #[cfg(all(feature = "serde", any(feature = "categories", feature = "users")))]
        let serde_type_attrs = serde_type_attrs();

        #[cfg(all(feature = "serde", any(feature = "categories", feature = "users")))]
        let config = config.type_attribute(serde_type_attrs.0, serde_type_attrs.1);

        config.compile(&[f], &["src"])?;
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
