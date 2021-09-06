build: export LUA_LIB_NAME=lua
build: export LUA_LIB=$(CURDIR)/lua/lua5.1/
build: export LUA_INC=$(CURDIR)/lua/lua5.1/include
build:
	cargo build --features hot-reload
	powershell copy target/debug/dcs_grpc_server.dll target/debug/dcs_grpc_server_hot_reload.dll

watch: export LUA_LIB_NAME=lua
watch: export LUA_LIB=$(CURDIR)/lua/lua5.1/
watch: export LUA_INC=$(CURDIR)/lua/lua5.1/include
watch:
	cargo watch -x "check --features hot-reload"

test: export LUA_LIB_NAME=lua
test: export LUA_LIB=$(CURDIR)/lua/lua5.1/
test: export LUA_INC=$(CURDIR)/lua/lua5.1/include
test:
	cargo test
