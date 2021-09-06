package.cpath = package.cpath..lfs.writedir()..[[Mods\tech\DCS-gRPC\?.dll;]]
GRPC = { basePath = lfs.writedir()..[[Scripts\DCS-gRPC\]] }

local luaPath = GRPC.basePath..[[grpc.lua]]
local f = assert(loadfile(luaPath))

if f == nil then
  error("[GRPC]: Could not load "..luaPath)
else
  f()
end