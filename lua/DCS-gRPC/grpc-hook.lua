-- note: the hook's load will only fire after the mission loaded.
local function load()
  log.write("[GRPC-Hook]", log.INFO, "mission loaded, setting up gRPC listener ...")

  -- Let DCS know where to find the DLLs
  if not string.find(package.cpath, GRPC.dllPath) then
    package.cpath = package.cpath .. GRPC.dllPath .. [[?.dll;]]
  end

  local ok, grpc = pcall(require, "dcs_grpc_hot_reload")
  if ok then
    log.write("[GRPC-Hook]", log.INFO, "loaded hot reload version")
  else
    grpc = require("dcs_grpc")
  end

  _G.grpc = grpc
  assert(pcall(assert(loadfile(GRPC.luaPath .. [[grpc.lua]]))))

  log.write("[GRPC-Hook]", log.INFO, "gRPC listener set up.")
end

local handler = {}

function handler.onMissionLoadEnd()
  local ok, err = pcall(load)
  if not ok then
    log.write("[GRPC-Hook]", log.ERROR, "Failed to set up gRPC listener: "..tostring(err))
  end
end

function handler.onSimulationFrame()
  if GRPC.onSimulationFrame then
    GRPC.onSimulationFrame()
  end
end

function handler.onSimulationStop()
  log.write("[GRPC-Hook]", log.INFO, "simulation stopped, shutting down gRPC listener ...")

  GRPC.stop()
  grpc = nil
end

function handler.onPlayerTrySendChat(playerID, msg)
  -- note: currently `all` (third parameter) will always `=true` regardless if the target is to the coalition/team
  --        or to everybody. When ED fixes this, implementation should determine the dcs.common.v0.Coalition

  grpc.event({
    time = DCS.getModelTime(),
    event = {
      type = "playerSendChat",
      playerId = playerID,
      message = msg
    },
  })

  return msg
end

function handler.onPlayerTryConnect(addr, name, ucid, id)
  grpc.event({
    time = DCS.getModelTime(),
    event = {
      type = "connect",
      addr = addr,
      name = name,
      ucid = ucid,
      id = id,
    },
  })
  -- not returning `true` here to allow other scripts to handle this hook
end

function handler.onPlayerDisconnect(id, reason)
  grpc.event({
    time = DCS.getModelTime(),
    event = {
      type = "disconnect",
      id = id,
      reason = reason + 1, -- Increment for non zero-indexed gRPC enum
    },
  })
end

DCS.setUserCallbacks(handler)