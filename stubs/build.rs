fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../protos");

    tonic_build::configure()
        .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .type_attribute("dcs.mission.Event.event", "#[serde(tag = \"type\")]")
        .field_attribute(
            "dcs.mission.Event.MarkAddEvent.visibility",
            "#[serde(flatten)]",
        )
        .field_attribute(
            "dcs.mission.Event.MarkChangeEvent.visibility",
            "#[serde(flatten)]",
        )
        .field_attribute(
            "dcs.mission.Event.MarkRemoveEvent.visibility",
            "#[serde(flatten)]",
        )
        .build_server(cfg!(feature = "server"))
        .build_client(cfg!(feature = "client"))
        .compile(&["../protos/dcs.proto"], &["../protos"])?;
    Ok(())
}
