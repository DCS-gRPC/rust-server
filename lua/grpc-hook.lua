package.cpath = package.cpath..lfs.writedir()..[[Mods\tech\DCS-gRPC\?.dll;]]

local function init()
  log.write("[GRPC-Hook]", log.INFO, "Initializing ...")

  local handler = {}

  function handler.onMissionLoadEnd()
    log.write("[GRPC-Hook]", log.INFO, "mission loaded, setting up gRPC listener ...")

    _G.GRPC = { basePath = lfs.writedir()..[[Scripts\DCS-gRPC\]] }

    local luaPath = _G.GRPC.basePath..[[grpc.lua]]
    local f = assert(loadfile(luaPath))

    if f == nil then
      error("[GRPC-Hook]: Could not load "..luaPath)
    else
      f()
    end
  end

  function handler.onSimulationFrame()
    if GRPC.onSimulationFrame then
      GRPC.onSimulationFrame()
    end
  end

  function handler.onSimulationStop()
    log.write("[GRPC-Hook]", log.INFO, "simulation stopped, shutting down gRPC listener ...")

    _G.GRPC.stop()
    _G.GRPC = nil
  end

  function handler.onPlayerTrySendChat(playerID, msg, all)
    _G.GRPC.onChatMessage(playerID, msg, all)
    return msg
  end

  DCS.setUserCallbacks(handler)

  log.write("[GRPC-Hook]", log.INFO, "Initialized ...")
end

local ok, err = pcall(init)
if not ok then
  log.write("[GRPC-Hook]", log.ERROR, "Failed to Initialize: "..tostring(err))
end
