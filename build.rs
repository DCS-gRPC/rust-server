fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .field_attribute("dcs.Event.event", "#[serde(flatten)]")
        .build_server(true)
        .compile(&["proto/dcs.proto"], &["proto"])?;
    Ok(())
}
