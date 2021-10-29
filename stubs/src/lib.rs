tonic::include_proto!("dcs");

pub mod atmosphere {
    tonic::include_proto!("dcs.atmosphere");
}

pub mod coalition {
    tonic::include_proto!("dcs.coalition");
}

pub mod controller {
    tonic::include_proto!("dcs.controller");
}

pub mod custom {
    tonic::include_proto!("dcs.custom");
}

pub mod group {
    tonic::include_proto!("dcs.group");
}

pub mod hook {
    tonic::include_proto!("dcs.hook");
}

pub mod mission {
    tonic::include_proto!("dcs.mission");
}

pub mod timer {
    tonic::include_proto!("dcs.timer");
}

pub mod trigger {
    tonic::include_proto!("dcs.trigger");
}

pub mod unit {
    tonic::include_proto!("dcs.unit");
}

pub mod world {
    tonic::include_proto!("dcs.world");
}
