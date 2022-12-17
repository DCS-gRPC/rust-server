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

To make the gRPC server available in the mission scripting environment, add the following line to your `MissionScripting.lua` file that is found by default in the DCS World install folder at `C:\Program Files\Eagle Dynamics\DCS World\Scripts\MissionScripting.lua`. If you installed the server in another location then look for the `Scripts\MissionScripting.lua` file in there.

Note: The following file is in `diff` format to highlight the line that has been added. **Do note include** the `+` symbol when you paste into the `MissionScripting.lua` file.

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

### Running DCS-gRPC

There are two ways of running DCS-gRPC. One way allows it to run regardless of what mission is running and the other
means that DCS-gRPC will _only_ run if the mission scripting itself enables it.

### Running regardless of mission

Create the file `Saved Games\DCS\Config\dcs-grpc.lua` and add the line below

```lua
autostart = true
```

As well as this you can set other options in this file. These are listed below:

```lua
-- Whether the `Eval` method is enabled or not.
evalEnabled = false

-- The host the gRPC listens on (use "0.0.0.0" to listen on all IP addresses of the host).
host = "127.0.0.1"

-- The port to listen on.
port = 50051

-- Whether debug logging is enabled or not.
debug = false

-- Limit of calls per second that are executed inside of the mission scripting environment.
throughputLimit = 600

-- The default TTS provider to use if a TTS request does not explicitly specify another one.
tts.defaultProvider = "win"

-- Your Azure access key.
tts.provider.azure.key = "..."

-- The Azure region to use (see https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/regions).
tts.provider.azure.region = "westeurope"

-- The default Azure voice to use (see https://learn.microsoft.com/azure/cognitive-services/speech-service/language-support).
tts.provider.azure.defaultVoice = "en-US-AriaNeural"

-- Your AWS key.
tts.provider.aws.key = "..."

-- Your AWS secret key.
tts.provider.aws.secret = "..."

-- Your AWS region (see https://docs.aws.amazon.com/general/latest/gr/pol.html).
tts.provider.aws.region = "eu-central-1"

-- The default AWS voice to use (see https://docs.aws.amazon.com/polly/latest/dg/voicelist.html).
tts.provider.aws.defaultVoice = "Brian"

-- Your Google Cloudd access key.
tts.provider.gcloud.key = "..."

-- The default Google Cloud voice to use (see https://cloud.google.com/text-to-speech/docs/voices).
tts.provider.gcloud.defaultVoice = "en-GB-Neural2-A"

-- The default Windows voice to use (see https://support.microsoft.com/en-us/windows/appendix-a-supported-languages-and-voices-4486e345-7730-53da-fcfe-55cc64300f01).
-- Requires at least Windows Server 2019 to work properly.
tts.provider.win.defaultVoice = "David"

-- Your SRS server's address.
srs.addr = "127.0.0.1:5002"
```

Once you have done this start the DCS server and skip to the "Confirming that DCS-gRPC is running" section of this
README.

### Running only if the mission scripting enables it

Make sure that the file `Saved Games\DCS\Config\dcs-grpc.lua` does not exist (Delete if it does).

Add the following code to your mission. This will start the DCS-gRPC server. You can add this code to a `DO SCRIPT`
trigger in your .miz file or you can add this code to an existing lua file that your mission may be running.

```lua
-- Load the gRPC server into the mission
GRPC.load()
```

As well as this you can set other options in the script _before_ `GRPC.Load()` . These are listed below:

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
```

For example:

```lua
GRPC.host = '0.0.0.0'
GRPC.load()
```

### Confirming that DCS-gRPC is running

To confirm that the server is running check the `\Logs\dcs.log` file and look for entries prefixed with `GRPC`.
You can also check for the present of a `\Logs\grpc.log` file.

The server will be running on port 50051 by default.

## Lua API

`DCS-gRPC` provides the following Lua APIs to interact with the server from within Lua.

- `GRPC.tts(ssml, frequency[, options])` - Synthesize text (`ssml`; SSML tags supported) to speech and transmit it over SRS on the `frequency` with the following optional `options` (and their defaults):
    ```lua
    {
        -- The plain text without any transformations made to it for the purpose of getting it spoken out
        -- as desired (no SSML tags, no FOUR NINER instead of 49, ...). Even though this field is
        -- optional, please consider providing it as it can be used to display the spoken text to players
        -- with hearing impairments.
        plaintext = null, -- e.g. `= "Hello Pilot"`

        -- Name of the SRS client.
        srsClientName = "DCS-gRPC",

        -- The origin of the transmission. Relevant if the SRS server has "Line of
        -- Sight" and/or "Distance Limit" enabled.
        position = {
            lat = 0.0,
            lon = 0.0,
            alt = 0.0, -- in meters
        },

        -- The coalition of the transmission. Relevant if the SRS server has "Secure
        -- Coalition Radios" enabled. Supported values are: `blue` and `red`. Defaults
        -- to being spectator if not specified.
        coalition = null,

        -- TTS provider to be use. Defaults to the one configured in your config or to Windows'
        -- built-in TTS. Examples:
        -- `= { aws = {} }` / `= { aws = { voice = "..." } }` enable AWS TTS
        -- `= { azure = {} }` / `= { azure = { voice = "..." } }` enable Azure TTS
        -- `= { gcloud = {} }` / `= { gcloud = { voice = "..." } }` enable Google Cloud TTS
        -- `= { win = {} }` / `= { win = { voice = "..." } }` enable Windows TTS
        provider = null,
    }
    ```

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

- Rust `>=1.64`
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

### Prepare DCS

For development:

- update your `MissionScripting.lua` to load `grpc-mission.lua` from your local clone, e.g.:
  ```diff
  - dofile(lfs.writedir()..[[Scripts\DCS-gRPC\grpc-mission.lua]])
  + dofile([[C:\Development\DCS-gRPC\rust-server\lua\DCS-gRPC\grpc-mission.lua]])
  ```
- add custom `dllPath` and `luaPath` settings to your `Saved Games\DCS\Config\dcs-grpc.lua`:
  ```lua
  dllPath = [[C:\Development\DCS-gRPC\rust-server\target\debug\]]
  luaPath = [[C:\Development\DCS-gRPC\rust-server\lua\DCS-gRPC\]]
  ```
- copy the hook script from `lua\Hooks\DCS-gRPC.lua` to `Scripts\Hooks\DCS-gRPC.lua`

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
cargo build -p dcs-grpc-repl
# Make sure your DCS mission is running
cargo run -p dcs-grpc-repl
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

### Contributions

This repository is powered by GitHub Actions for the Continuous Integration (CI) services. The same CI checks would be
triggered and executed as you push code to your forked repository, and providing early feedback before a maintainer
executes a manual execution on the pull request.

### Troublshooting

#### Linker Error 1104

If you see `LINK : fatal error LNK1104: cannot open file` when running
`cargo build` make sure that there is no running DCS mission as that
locks the DLL files. Exit the mission (You do *not* have to exit DCS)
then re-run the command before restarting the mission.
