fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../protos/dcs");

    tonic_build::configure()
        .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .type_attribute(
            "dcs.mission.StreamEventsResponse.event",
            "#[serde(tag = \"type\")]",
        )
        .field_attribute(
            "dcs.mission.StreamEventsResponse.MarkAddEvent.visibility",
            "#[serde(flatten)]",
        )
        .field_attribute(
            "dcs.mission.StreamEventsResponse.MarkChangeEvent.visibility",
            "#[serde(flatten)]",
        )
        .field_attribute(
            "dcs.mission.StreamEventsResponse.MarkRemoveEvent.visibility",
            "#[serde(flatten)]",
        )
        .build_server(cfg!(feature = "server"))
        .build_client(cfg!(feature = "client"))
        .compile(&["../protos/dcs/dcs.proto"], &["../protos"])?;
    Ok(())
}
