fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/proto");

    #[allow(unused_mut)]
    let mut protos = Vec::new();

    dbg!("created vec");

    #[cfg(any(feature = "categories"))]
    protos.push("src/proto/common.proto");

    #[cfg(feature = "categories")]
    protos.push("src/proto/category.proto");

    if !protos.is_empty() {
        #[cfg(feature = "rpc")]
        build_with_rpc(&protos)?;

        #[cfg(not(feature = "rpc"))]
        build_no_rpc(&protos)?;
    }
    Ok(())
}

#[cfg(feature = "rpc")]
fn build_with_rpc(protos: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let config = tonic_build::configure().build_server(true);

    #[cfg(feature = "serde")]
    let serde_type_attrs = serde_type_attrs();

    #[cfg(feature = "serde")]
    let config = config.type_attribute(serde_type_attrs.0, serde_type_attrs.1);

    config.compile(protos, &["src"])?;
    Ok(())
}

#[cfg(feature = "serde")]
fn serde_type_attrs() -> (impl AsRef<str>, impl AsRef<str>) {
    (
        ".",
        "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all = \"snake_case\")]",
    )
}

#[cfg(not(feature = "rpc"))]
fn build_no_rpc(protos: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();

    #[cfg(feature = "serde")]
    let serde_type_attrs = serde_type_attrs();

    #[cfg(feature = "serde")]
    config.type_attribute(serde_type_attrs.0, serde_type_attrs.1);

    config
        .compile_well_known_types()
        .compile_protos(protos, &["src/"])?;

    Ok(())
}
