fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("PROTOC", protoc_bundled::PROTOC);
    std::env::set_var("PROTOC_INCLUDE", protoc_bundled::PROTOC_INCLUDE);

    println!("cargo:rerun-if-changed=../protos/dcs");

    tonic_build::configure()
        .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
        .type_attribute(".", "#[serde(rename_all = \"camelCase\")]")
        .type_attribute(
            "dcs.mission.v0.StreamEventsResponse.event",
            "#[serde(tag = \"type\")]",
        )
        .field_attribute(
            "dcs.mission.v0.StreamEventsResponse.MarkAddEvent.visibility",
            "#[serde(flatten)]",
        )
        .field_attribute(
            "dcs.mission.v0.StreamEventsResponse.MarkChangeEvent.visibility",
            "#[serde(flatten)]",
        )
        .field_attribute(
            "dcs.mission.v0.StreamEventsResponse.MarkRemoveEvent.visibility",
            "#[serde(flatten)]",
        )
        .field_attribute(
            "dcs.mission.v0.AddMissionCommandRequest.details",
            r#"#[serde(with = "crate::utils::proto_struct")]"#,
        )
        .field_attribute(
            "dcs.mission.v0.StreamEventsResponse.MissionCommandEvent.details",
            r#"#[serde(with = "crate::utils::proto_struct")]"#,
        )
        .field_attribute(
            "dcs.mission.v0.AddCoalitionCommandRequest.details",
            r#"#[serde(with = "crate::utils::proto_struct")]"#,
        )
        .field_attribute(
            "dcs.mission.v0.StreamEventsResponse.CoalitionCommandEvent.details",
            r#"#[serde(with = "crate::utils::proto_struct")]"#,
        )
        .field_attribute(
            "dcs.mission.v0.AddGroupCommandRequest.details",
            r#"#[serde(with = "crate::utils::proto_struct")]"#,
        )
        .field_attribute(
            "dcs.mission.v0.StreamEventsResponse.GroupCommandEvent.details",
            r#"#[serde(with = "crate::utils::proto_struct")]"#,
        )
        .build_server(cfg!(feature = "server"))
        .build_client(cfg!(feature = "client"))
        .compile(&["../protos/dcs/dcs.proto"], &["../protos"])?;
    Ok(())
}
