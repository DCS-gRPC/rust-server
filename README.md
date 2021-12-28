# DCS gRPC Server

DCS gRPC is an RPC (Remote Procedure Call) server that allows network clients to interact with a currently running
mission on a DCS server.

## Installation

### Download

Download the latest version of the server from the [Releases](https://github.com/DCS-gRPC/rust-server/releases) and
extract the zip file into your DCS Server directory.

This is typically found in `C:\Users\USERNAME\Saved Games\DCS.openbeta_server`.
Once extracted you will have a `Scripts\DCS-gRPC` folder, a `Mods\Tech\DCS-gRPC` folder, and a
`Scripts\Hooks\DCS-gRPC.lua` file in your server folder. As well as these scripts there will be a `Docs/DCS-gRPC`
folder containing documentation and a `Tools/DCS-gRPC` folder containing client tools.

### Prepare DCS

To make the gRPC server available in the mission scripting environment, add the following line to your `DCS World\Scripts\MissionScripting.lua`.

```diff
  --Initialization script for the Mission lua Environment (SSE)

  dofile('Scripts/ScriptingSystem.lua')
+ dofile(lfs.writedir()..[[Scripts\DCS-gRPC\grpc-mission.lua]])

  --Sanitize Mission Scripting environment
  --This makes unavailable some unsecure functions.
  --Mission downloaded from server to client may contain potentialy harmful lua code that may use these functions.
  --You can remove the code below and make availble these functions at your own risk.

  local function sanitizeModule(name)
    _G[name] = nil
    package.loaded[name] = nil
  end

  do
    sanitizeModule('os')
    sanitizeModule('io')
    sanitizeModule('lfs')
    _G['require'] = nil
    _G['loadlib'] = nil
    _G['package'] = nil
  end
```

### Prepare Mission

Add the following code to your mission. This will start the DCS-gRPC server. You can add this code to a `DO SCRIPT`
trigger in your .miz file or you can add this code to an existing lua file that your mission may be running.

```lua
-- Optional: Change settings if desired, e.g.:
GRPC.debug = true

-- Required: Load the gRPC server into the mission
GRPC.load()
```

### Settings

The behaviour of the gRPC server can be fine-tuned using various settings that can be set on the `GRPC` global (before the `grpc.lua` is executed). The available settings and their defaults are:

```lua
-- Whether the `Eval` method is enabled or not.
GRPC.evalEnabled = false

-- The host the gRPC listens on (use "0.0.0.0" to listen on all IP addresses of the host).
GRPC.host = '127.0.0.1'

-- The port to listen on.
GRPC.port = 50051

-- Whether debug logging is enabled or not.
GRPC.debug = false

-- Limit of calls per second that are executed inside of the mission scripting environment.
GRPC.throughputLimit = 600

-- Whether the gRPC server should be automatically started for each mission on the DCS instance
-- When `true`, it is not necessary to run `GRPC.load()` inside of a mission anymore.
GRPC.autostart = false
```

Settings can either be set:
- above `GRPC.load()` inside of a mission,
  ```lua
  GRPC.debug = true
  GRPC.autostart = true
  GRPC.load()
  ```

- inside a `Saved Games\DCS\Config\dcs-grpc.lua` file (useful if run multiple servers in parallel), or

  `Saved Games\DCS\Config\dcs-grpc.lua`:
  ```lua
  GRPC.debug = true
  GRPC.autostart = true
  ```

- globally inside of the `MissionScripting.lua`
  ```diff
    --Initialization script for the Mission lua Environment (SSE)

    dofile('Scripts/ScriptingSystem.lua')
  +
  + GRPC = {
  +   throughputLimit = 200,
  +   autostart = true
  + }
  + dofile(lfs.writedir()..[[Scripts\DCS-gRPC\grpc-mission.lua]])

    ...
  ```

### Confirmation

To confirm that the server is running check the `\Logs\dcs.log` file and look for entries prefixed with `GRPC`.
You can also check for the present of a `\Logs\grpc.log` file.

The server will be running on port 50051

## Client Development

`DCS-gRPC`, as the name implies, uses the [gRPC](https://grpc.io/) framework to handle communication between clients
and the server. gRPC supports a wide variety of languages which allows you to develop clients in the languages of
your choice.

In order to develop clients for `DCS-gRPC` you must be familiar with gRPC concepts so we recommend reading the
[gRPC documentation](https://grpc.io/docs/) for your language.

The gRPC .proto files are available in the `Docs/DCS-gRPC` folder and also available in the Github repo

## Server Development

The following section is only applicable to people who want to developer the DCS-gRPC server itself.

### Build Dependencies

- Rust `>=1.56`
- `rustfmt` (`rustup component add rustfmt`)

### Build

```
make build
```

You may need to use the following in powershell

```
cargo build
```

Or if you want to use the hot reloading DLL (this is the same as `make build`):

```
cargo build --features hot-reload
copy target/debug/dcs_grpc.dll target/debug/dcs_grpc_hot_reload.dll
```

### Prepare Mission

For development, update the previously added line in `DCS World\Scripts\MissionScripting.lua` to point to your checked out repository of the gRPC server:

```diff
- dofile(lfs.writedir()..[[Scripts\DCS-gRPC\grpc-mission.lua]])
+ GRPC = {
+ 	dllPath = [[C:\Development\DCS-gRPC\rust-server\target\debug\]],
+ 	luaPath = [[C:\Development\DCS-gRPC\rust-server\lua\]]
+ }
+ dofile(GRPC.luaPath .. [[grpc-mission.lua]])
```

### Debugging

- Search for `[GRPC]` in the DCS logs
- Consult the gRPC Server logs at `Saved Games\DCS.openbeta\Logs\gRPC.log`

Test the running server via [grpcurl](https://github.com/fullstorydev/grpcurl): (Remove the `.exe` when running on Linux)

```bash
grpcurl.exe -plaintext -import-path ./protos -proto ./protos/dcs/dcs.proto -d '{\"text\": \"Works!\", \"display_time\": 10, \"clear_view\": false}' 127.0.0.1:50051 dcs.trigger.v0.TriggerService/OutText
```

or watch the mission event stream via:

```bash
grpcurl.exe -plaintext -import-path ./protos -proto ./protos/dcs/dcs.proto -d '{}' 127.0.0.1:50051 dcs.mission.v0.MissionService/StreamEvents
```

#### REPL

`DCS-gRPC` provides the facility to directly run lua code inside the mission scripting environment. This feature is
mainly intended for development and is disabled by default. You can enable it via the `GRPC` settings
(See `Settings` section above)

To build and run the repl run the following commands

```bash
cargo build --bin repl
# Make sure your DCS mission is running
cargo run --bin repl
```

Note that the REPL is hardcoded to connect to localhost on the default port

Once connected you can enter lua code to execute. Prefix the lua with `return` to have the lua code return a value to
the client. For example:

```lua
return Group.getByName('Aerial-1')
= {
    "id_": 1
}

return Group.getByName('Aerial-1'):getName()
= Aerial-1
```

The REPL is also available in the release and can be run by running `Tools/DCS-gRPC/repl.exe`

### Troublshooting

#### Linker Error 1104

If you see `LINK : fatal error LNK1104: cannot open file` when running
`cargo build` make sure that there is no running DCS mission as that
locks the DLL files. Exit the mission (You do *not* have to exit DCS)
then re-run the command before restarting the mission.
