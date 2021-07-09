# DCS gRPC Server

## Development

### Build Dependencies

- Rust `>=1.39`
- `rustfmt` (`rustup component add rustfmt`)

### Build

```
make build
```

You may need to use the following in powershell

```
$env:LUA_LIB_NAME="lua"
$env:LUA_LIB=(Get-Item -Path ".\").FullName+"/lua/lua5.1/"
$env:LUA_INC=(Get-Item -Path ".\").FullName+"/lua/lua5.1/include"
cargo build
```

### Mission Setup

Remove the sanitation of `require` and the `lfs` from your `DCS World\Scripts\MissionScripting.lua`. After this change, it is recommended to only run missions that you trust.

```diff
do
 	sanitizeModule('os')
	sanitizeModule('io')
-	sanitizeModule('lfs')
-	require = nil
+	-- sanitizeModule('lfs')
+	-- require = nil
	loadlib = nil
end
```

Add the following script to your mission (adjust the paths to match your repo location):

```lua
package.cpath = package.cpath..[[M:\Development\DCS-gRPC\rust-server\target\debug\?.dll;]]
GRPC = { basePath = [[M:\Development\DCS-gRPC\rust-server\lua\]] }
dofile(GRPC.basePath .. [[grpc.lua]])
```

### Debugging

- Seach for `[GRPC]` in the DCS logs
- Consult the gRPC Server logs at `Saved Games\DCS.openbeta\Logs\gRPC.log`

Test the running server via [grpcurl](https://github.com/fullstorydev/grpcurl):

```bash
grpcurl.exe -plaintext -import-path ./protos -proto ./protos/dcs_mission.proto -d '{\"text\": \"Works!\", \"display_time\": 10, \"clear_view\": false}' 127.0.0.1:50051 dcs.Mission/OutText
```

or watch the mission event stream via:

```bash
grpcurl.exe -plaintext -import-path ./protos -proto ./protos/dcs_mission.proto -d '{}' 127.0.0.1:50051 dcs.Mission/StreamEvents
```

### Troublshooting

### Linker Error 1104

If you see `LINK : fatal error LNK1104: cannot open file` when running
`cargo build` make sure that there is no running DCS mission as that
locks the DLL files. Exit the mission (You do *not* have to exit DCS)
then re-run the command before restarting the mission.