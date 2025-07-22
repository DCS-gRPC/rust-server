--
-- Hook actions
-- Docs: /DCS World/API/DCS_ControlAPI.html
--

local DCS = DCS
local Sim = Sim
local GRPC = GRPC
local net = net
local Export = Export

GRPC.methods.getMissionName = function()
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
     GRPC.success({name = DCS.getMissionName()})
  else
      GRPC.success({name = Sim.getMissionName()})
  end
end

GRPC.methods.getMissionFilename = function()
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
    return GRPC.success({name = DCS.getMissionFilename()})
  else
    return GRPC.success({name = Sim.getMissionFilename()})
  end
end

GRPC.methods.getMissionDescription = function()
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
    return GRPC.success({name = DCS.getMissionDescription()})
  else
    return GRPC.success({name = Sim.getMissionDescription()})
  end
end

GRPC.methods.reloadCurrentMission = function()
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
    net.load_mission(DCS.getMissionFilename())
  else
    net.load_mission(Sim.getMissionFilename())
  end
  return GRPC.success({})
end

GRPC.methods.loadNextMission = function()
  return GRPC.success({loaded = net.load_next_mission()})
end

GRPC.methods.loadMission = function(params)
  return GRPC.success({loaded = net.load_mission(params.fileName)})
end

GRPC.methods.getPaused = function()
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
    return GRPC.success({paused = DCS.getPause()})
  else
    return GRPC.success({paused = Sim.getPause()})
  end
end

GRPC.methods.setPaused = function(params)
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
    DCS.setPause(params.paused)
  else
    Sim.setPause(params.paused)
  end
  return GRPC.success({})
end

GRPC.methods.stopMission = function()
   if DCS then --Backwards compatibility with DCS 2.9.17 and before
    DCS.stopMission()
  else
    Sim.stopMission()
  end
  return GRPC.success({})
end

GRPC.methods.exitProcess = function()
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
    DCS.exitProcess()
  else
    Sim.exitProcess()
  end
  return GRPC.success({})
end

GRPC.methods.hookEval = function(params)
  local fn, err = loadstring(params.lua)
  if not fn then
    return GRPC.error("Failed to load Lua code: "..err)
  end

  local ok, result = pcall(fn)
  if not ok then
    return GRPC.error("Failed to execute Lua code: "..result)
  end

  return GRPC.success(net.lua2json(result))
end

GRPC.methods.isMultiplayer = function()
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
    return GRPC.success({multiplayer = DCS.isMultiplayer()})
  else
    return GRPC.success({multiplayer = Sim.isMultiplayer()})
  end
end

GRPC.methods.isServer = function()
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
    return GRPC.success({server = DCS.isServer()})
  else
    return GRPC.success({server = Sim.isServer()})
  end
end

GRPC.methods.banPlayer = function(params)
  if params.id == 1 then
    return GRPC.errorInvalidArgument("Cannot ban the server user")
  end

  local player_id = net.get_player_info(params.id, "id")

  if not player_id then
    return GRPC.errorNotFound("Could not find player with the ID of " .. params.id)
  end

  return GRPC.success({banned = net.banlist_add(params.id, params.period, params.reason)})
end

GRPC.methods.unbanPlayer = function(params)
  return GRPC.success({unbanned = net.banlist_remove(params.ucid)})
end

GRPC.methods.getBannedPlayers = function()
  local result = {}

  for i, detail in ipairs(net.banlist_get()) do
    result[i] = {
      ucid = detail.ucid,
      ipAddress = detail.ipaddr,
      playerName = detail.name,
      reason = detail.reason,
      bannedFrom = detail.banned_from,
      bannedUntil = detail.banned_until
    }
  end

  return GRPC.success({bans = result})
end

GRPC.methods.getUnitType = function(params)
  -- https://wiki.hoggitworld.com/view/DCS_func_getUnitType
  local unit_type
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
    unit_type = DCS.getUnitType(params.id)
  else
    unit_type = Sim.getUnitType(params.id)
  end
  -- getUnitType returns an empty string if the unit doesn't exist, ensure we catch eventual nils too
  if unit_type == nil or unit_type == "" then
    return GRPC.errorNotFound("unit `" .. tostring(params.id) .. "` does not exist")
  end

  return GRPC.success({type = unit_type})
end

GRPC.methods.getRealTime = function()
  -- https://wiki.hoggitworld.com/view/DCS_func_getRealTime
  if DCS then --Backwards compatibility with DCS 2.9.17 and before
    return GRPC.success({time = DCS.getRealTime()})
  else
    return GRPC.success({time = Sim.getRealTime()})
  end
end

GRPC.methods.getBallisticsCount = function()
  local ballistics = Export.LoGetWorldObjects("ballistic")
  local count = 0
  for _ in pairs(ballistics) do count = count + 1 end
  return GRPC.success({count = count})
end

