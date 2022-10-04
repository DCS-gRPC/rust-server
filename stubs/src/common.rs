pub mod v0 {
    use std::ops::Neg;

    tonic::include_proto!("dcs.common.v0");

    #[derive(Default, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct RawTransform {
        pub position: Option<Position>,
        pub position_north: Option<Vector>,
        pub forward: Option<Vector>,
        pub right: Option<Vector>,
        pub up: Option<Vector>,
        pub velocity: Option<Vector>,
    }

    pub(crate) struct Transform {
        pub position: Position,
        pub orientation: Orientation,
        pub velocity: Velocity,
    }

    impl From<RawTransform> for Transform {
        fn from(raw: RawTransform) -> Self {
            let RawTransform {
                position,
                position_north,
                forward,
                right,
                up,
                velocity,
            } = raw;
            let position = position.unwrap_or_default();
            let position_north = position_north.unwrap_or_default();
            let forward = forward.unwrap_or_default();
            let right = right.unwrap_or_default();
            let up = up.unwrap_or_default();
            let velocity = velocity.unwrap_or_default();

            let projection_error =
                (position_north.z - position.u).atan2(position_north.x - position.v);
            let heading = forward.z.atan2(forward.x);

            let orientation = Orientation {
                heading: {
                    let heading = heading.to_degrees();
                    if heading < 0.0 {
                        heading + 360.0
                    } else {
                        heading
                    }
                },
                yaw: (heading - projection_error).to_degrees(),
                roll: right.y.asin().neg().to_degrees(),
                pitch: forward.y.asin().to_degrees(),
                forward: Some(forward),
                right: Some(right),
                up: Some(up),
            };

            let velocity = Velocity {
                heading: {
                    let heading = velocity.z.atan2(velocity.x).to_degrees();
                    if heading < 0.0 {
                        heading + 360.0
                    } else {
                        heading
                    }
                },
                speed: (velocity.x.powi(2) + velocity.z.powi(2)).sqrt(),
                velocity: Some(velocity),
            };

            Transform {
                position,
                orientation,
                velocity,
            }
        }
    }

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct UnitIntermediate {
        id: u32,
        name: String,
        callsign: String,
        coalition: i32,
        r#type: String,
        player_name: Option<String>,
        group: Option<Group>,
        number_in_group: u32,
        raw_transform: Option<RawTransform>,
    }

    impl From<UnitIntermediate> for Unit {
        fn from(i: UnitIntermediate) -> Self {
            let UnitIntermediate {
                id,
                name,
                callsign,
                coalition,
                r#type,
                player_name,
                group,
                number_in_group,
                raw_transform,
            } = i;
            let transform = Transform::from(raw_transform.unwrap_or_default());
            Unit {
                id,
                name,
                callsign,
                coalition,
                r#type,
                position: Some(transform.position),
                orientation: Some(transform.orientation),
                velocity: Some(transform.velocity),
                player_name,
                group,
                number_in_group,
            }
        }
    }
}
