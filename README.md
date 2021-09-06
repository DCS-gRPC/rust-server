# DCS gRPC Server

DCS gRPC is an RPC (Remote Procedure Call) server that allows network clients to interact with a currently running
mission on a DCS server.

## Installation

### Download

Download the latest version of the server from the [Releases](https://github.com/DCS-gRPC/rust-server/releases) and
extract the zip file into your DCS Server directory. This is typically found in
`C:\Users\USERNAME\Saved Games\DCS.openbeta_server`. Once extracted you will have a `Scripts\DCS-gRPC` folder,
a `Mods\Tech\DCS-gRPC` folder, and a `Scripts\Hooks\DCS-gRPC.lua` file.

### MissionScripting Sanitisation Removal

DCS-gRPC requires the removal of sanitisation features in DCS scripting.

Remove the sanitation of `require` and the `lfs` from your `DCS World\Scripts\MissionScripting.lua`.

After this change, it is recommended to only run missions and scripts that you trust.

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

### Mission Editing

Create a trigger of type `MISSION START` and add a `DO SCRIPT FILE` action loading `Scripts\DCS-gRPC\grpc-mission.lua`.

### Confirmation

To confirm that the server is running check the `\Logs\dcs.log` file and look for entries prefixed with `GRPC`.
You can also check for the present of a `\Logs\grpc.log` file.

The server will be running on port 50051

## Development

The following section is only applicable to people who want to developer the DCS-gRPC server itself.

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

### Link files

To always have the latest development changes

DCS-gRPC requires the removal of sanitisation features in DCS scripting.

Remove the sanitation of `require` and the `lfs` from your `DCS World\Scripts\MissionScripting.lua`.

After this change, it is recommended to only run missions and scripts that you trust.

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

Instead of copying the files to their respective destination as done in the normal [Installation](#installation), it is recommended to create symbolic links for development instead.

This can be done using powershell. Before running the commands, update the paths accordingly.

Build to make sure all files exist:

```bash
make build
```

Create directories and links:

```ps1
New-Item -ItemType SymbolicLink -Path "M:\Saved Games\DCS.openbeta\Scripts\DCS-gRPC" -Value "M:\Development\DCS-gRPC\rust-server\lua"
New-Item -ItemType SymbolicLink -Path "M:\Saved Games\DCS.openbeta\Scripts\Hooks\DCS-gRPC.lua" -Value "M:\Development\DCS-gRPC\rust-server\lua\grpc-hook.lua"
New-Item -Path "M:\Saved Games\DCS.openbeta\Mods\Tech\DCS-gRPC" -ItemType "directory"
New-Item -ItemType SymbolicLink -Path "M:\Saved Games\DCS.openbeta\Mods\Tech\DCS-gRPC\dcs_grpc_server.dll" -Value "M:\Development\DCS-gRPC\rust-server\target\debug\dcs_grpc_server.dll"
```

### Mission Setup

Add `Scripts\DCS-gRPC\grpc-mission.lua` to your mission.

### Debugging

- Seach for `[GRPC]` in the DCS logs
- Consult the gRPC Server logs at `Saved Games\DCS.openbeta\Logs\gRPC.log`

Test the running server via [grpcurl](https://github.com/fullstorydev/grpcurl): (Remove the `.exe` when running on Linux)

```bash
grpcurl.exe -plaintext -import-path ./protos -proto ./protos/dcs.proto -d '{\"text\": \"Works!\", \"display_time\": 10, \"clear_view\": false}' 127.0.0.1:50051 dcs.Triggers/OutText
```

or watch the mission event stream via:

```bash
grpcurl.exe -plaintext -import-path ./protos -proto ./protos/dcs.proto -d '{}' 127.0.0.1:50051 dcs.Mission/StreamEvents
```

### Reload changes

To reload changes during development it is enough to restart a mission to reload changes related to the mission environment, but it is required to restart DCS to reload changes made to the hook environment.

### Troublshooting

#### Linker Error 1104

If you see `LINK : fatal error LNK1104: cannot open file` when running
`cargo build` make sure that there is no running DCS mission as that
locks the DLL files. Exit the mission (You do *not* have to exit DCS)
then re-run the command before restarting the mission.