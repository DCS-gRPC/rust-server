fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .type_attribute("dcs.Event.event", "#[serde(tag = \"type\")]")
        .field_attribute("dcs.Event.MarkAddEvent.visibility", "#[serde(flatten)]")
        .field_attribute("dcs.Event.MarkChangeEvent.visibility", "#[serde(flatten)]")
        .field_attribute("dcs.Event.MarkRemoveEvent.visibility", "#[serde(flatten)]")
        .build_server(true)
        .compile(&["proto/dcs.proto"], &["proto"])?;
    Ok(())
}
