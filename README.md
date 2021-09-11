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

Add the following code to your mission. This will start the DCS-gRPC server. You can add this code

- to a `DO SCRIPT FILE` trigger action loading `Scripts\DCS-gRPC\grpc-mission.lua`, or
- to a `DO SCRIPT` trigger action (or include it in your own scripts if you already have some) with the following script:

    ```lua
    package.cpath = package.cpath..lfs.writedir()..[[Mods\tech\DCS-gRPC\?.dll;]]
    GRPC = { basePath = lfs.writedir()..[[Scripts\DCS-gRPC\]] }

    local luaPath = GRPC.basePath .. [[grpc.lua]]
    local f = assert( loadfile(luaPath) )

    if f == nil then
      error ("[GRPC]: Could not load " .. luaPath )
    else
      f()
    end
    ```

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

### Mission Setup

Add the following script to your mission (adjust the paths to match your repo location):

```lua
package.cpath = package.cpath..[[C:\Development\DCS-gRPC\rust-server\target\debug\?.dll;]]
GRPC = { basePath = [[C:\Development\DCS-gRPC\rust-server\lua\]] }
dofile(GRPC.basePath
```

#### Link files

Instead of pointing your mission to your dev environment, you can also create symbolic links instead.
This can be done using powershell. Before running the commands, update the paths accordingly.

Build to make sure all files exist:

```bash
make build
```

Create directories and links:

```ps1
New-Item -ItemType SymbolicLink -Path "C:\Users\YOUR_USER\Saved Games\DCS.openbeta\Scripts\DCS-gRPC" -Value "C:\Development\DCS-gRPC\rust-server\rust-server\lua"
New-Item -ItemType SymbolicLink -Path "C:\Users\YOUR_USER\Saved Games\DCS.openbeta\Scripts\Hooks\DCS-gRPC.lua" -Value "C:\Development\DCS-gRPC\rust-server\rust-server\lua\grpc-hook.lua"
New-Item -Path "C:\Users\YOUR_USER\Saved Games\DCS.openbeta\Mods\Tech\DCS-gRPC" -ItemType "directory"
New-Item -ItemType SymbolicLink -Path "C:\Users\YOUR_USER\Saved Games\DCS.openbeta\Mods\Tech\DCS-gRPC\dcs_grpc_server.dll" -Value "C:\Development\DCS-gRPC\rust-server\rust-server\target\debug\dcs_grpc_server.dll"
New-Item -ItemType SymbolicLink -Path "C:\Users\YOUR_USER\Saved Games\DCS.openbeta\Mods\Tech\DCS-gRPC\dcs_grpc_server_hot_reload.dll" -Value "C:\Development\DCS-gRPC\rust-server\rust-server\target\debug\dcs_grpc_server_hot_reload.dll"
```

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

### Troublshooting

#### Linker Error 1104

If you see `LINK : fatal error LNK1104: cannot open file` when running
`cargo build` make sure that there is no running DCS mission as that
locks the DLL files. Exit the mission (You do *not* have to exit DCS)
then re-run the command before restarting the mission.