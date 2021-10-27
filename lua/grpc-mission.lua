-- Allow manually setting GRPC before this file is loaded.
if _G.GRPC == nil then
  GRPC = {
    basePath = lfs.writedir() .. [[Scripts\DCS-gRPC\]],
  }
end

-- Allow manually adding a DCS-gRPC DLL path.
if string.find(package.cpath, "DCS-gRPC") == nil then
  package.cpath = package.cpath .. lfs.writedir()..[[Mods\tech\DCS-gRPC\?.dll;]]
end

-- Load DLL before `require` gets sanitized.
local ok, grpc = pcall(require, "dcs_grpc_server_hot_reload")
if ok then
  env.info("[GRPC] loaded hot reload version")
else
  grpc = require("dcs_grpc_server")
end

function GRPC.load()
  local env = setmetatable({grpc = grpc}, {__index = _G})
  local f = setfenv(assert(loadfile(GRPC.basePath .. [[grpc.lua]])), env)
  f()
end
