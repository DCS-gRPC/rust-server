-- note: the hook's load will only fire after the mission loaded.
local function load()
  log.write("[GRPC-Hook]", log.INFO, "mission loaded, setting up gRPC listener ...")

  -- Let DCS know where to find the DLLs
  if not string.find(package.cpath, GRPC.dllPath) then
    package.cpath = package.cpath .. [[;]] .. GRPC.dllPath .. [[?.dll;]]
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
    log.write("[GRPC-Hook]", log.ERROR, "Failed to set up gRPC listener: " .. tostring(err))
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

-- None of these methods should return anything as doing so breaks other scripts attempting to
-- react to the hook as well.

function handler.onPlayerTrySendChat(playerID, msg)
  -- note: currently `all` (third parameter) will always `=true` regardless if the target is to the coalition/team
  --        or to everybody. When ED fixes this, implementation should determine the dcs.common.v0.Coalition

  local modelTime = 0
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
    modelTime = DCS.getModelTime()
  else
    modelTime = Sim.getModelTime()
  end

  grpc.event({
    time = modelTime,
    event = {
      type = "playerSendChat",
      playerId = playerID,
      message = msg
    },
  })
end

function handler.onPlayerTryConnect(addr, name, ucid, id)
  local modelTime = 0
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
    modelTime = DCS.getModelTime()
  else
    modelTime = Sim.getModelTime()
  end

  grpc.event({
    time = modelTime,
    event = {
      type = "connect",
      addr = addr,
      name = name,
      ucid = ucid,
      id = id,
    },
  })
end

function handler.onPlayerDisconnect(id, reason)
  local modelTime = 0
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
    modelTime = DCS.getModelTime()
  else
    modelTime = Sim.getModelTime()
  end

  grpc.event({
    time = modelTime,
    event = {
      type = "disconnect",
      id = id,
      reason = reason + 1, -- Increment for non zero-indexed gRPC enum
    },
  })
end

function handler.onPlayerChangeSlot(playerId)
  local playerInfo = net.get_player_info(playerId)
  local modelTime = 0
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
    modelTime = DCS.getModelTime() 
  else 
    modelTime = Sim.getModelTime() 
  end

  grpc.event({
    time = modelTime,
    event = {
      type = "playerChangeSlot",
      playerId = playerId,
      coalition = playerInfo.side + 1, -- offsetting for grpc COALITION enum
      slotId = playerInfo.slot
    },
  })
end

DCS.setUserCallbacks(handler)
