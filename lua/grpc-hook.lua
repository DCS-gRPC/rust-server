local function init()
  log.write("[GRPC]", log.ERROR, "Initializing hook ...")

  package.cpath = package.cpath..lfs.writedir()..[[Mods\tech\DCS-gRPC\?.dll;]]
  _G.GRPC = { basePath = lfs.writedir()..[[Scripts\DCS-gRPC\]] }

  local luaPath = _G.GRPC.basePath..[[grpc.lua]]
  local f = assert(loadfile(luaPath))

  if f == nil then
    error("[GRPC]: Could not load "..luaPath)
  else
    f()
  end

  log.write("[GRPC]", log.ERROR, "Hook initialized ...")
end

local ok, err = pcall(init)
if not ok then
  log.write("[GRPC]", log.ERROR, "Failed to init: "..tostring(err))
end
