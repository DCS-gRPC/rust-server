build:
	cargo build --features hot-reload
	powershell copy target/debug/dcs_grpc.dll target/debug/dcs_grpc_hot_reload.dll

watch:
	cargo watch -x "check --features hot-reload"

test:
	cargo test
