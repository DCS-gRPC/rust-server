build:
	cargo build --features hot-reload
	powershell copy target/debug/dcs_grpc_server.dll target/debug/dcs_grpc_server_hot_reload.dll

watch:
	cargo watch -x "check --features hot-reload"

test:
	cargo test
