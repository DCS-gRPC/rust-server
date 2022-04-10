fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=./protos/example");

    tonic_build::configure()
        .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .build_server(true)
        .build_client(false)
        .compile(&["./protos/example/example.proto"], &["./protos"])?;

    Ok(())
}
