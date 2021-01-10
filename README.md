# DCS gRPC Server

## Development

### Build Dependencies

- Rust `>=1.39`
- `rustfmt` (`rustup component add rustfmt`)

### Build

```
make build
```

### Mission Setup

Add the following script to your mission (adjust the paths to match your repo location):

```lua
package.cpath = package.cpath..[[M:\Development\DCS-gRPC\rust-server\target\debug\?.dll;]]
GRPC = { basePath = [[M:\Development\DCS-gRPC\rust-server\lua\]] }
dofile(GRPC.basePath .. [[grpc.lua]])
```

### Debugging

- Seach for `[GRPC]` in the DCS logs
- Consult the gRPC Server logs at `Saved Games\DCS.openbeta\Logs\gRPC.log`

Test the running server via:

```bash
grpcurl -plaintext -proto ./proto/dcs.proto -d "{\"text\": \"Works!\", \"display_time\": 10, \"clear_view\": false}" [::1]:50051 dcs.Mission/OutText
```