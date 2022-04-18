--
-- Hook actions
-- Docs: /DCS World/API/DCS_ControlAPI.html
--

local GRPC = GRPC

GRPC.methods.getMissionName = function()
  return GRPC.success({name = DCS.getMissionName()})
end

GRPC.methods.getMissionFilename = function()
  return GRPC.success({name = DCS.getMissionFilename()})
end

GRPC.methods.getMissionDescription = function()
  return GRPC.success({description = DCS.getMissionDescription()})
end

GRPC.methods.getPaused = function()
  return GRPC.success({paused = DCS.getPause()})
end

GRPC.methods.setPaused = function(params)
  DCS.setPause(params.paused)
  return GRPC.success(nil)
end

GRPC.methods.stopMission = function()
  DCS.stopMission()
  return GRPC.success(nil)
end

GRPC.methods.exitProcess = function()
  DCS.exitProcess()
  return GRPC.success(nil)
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

  return GRPC.success(result)
end

GRPC.methods.isMultiplayer = function()
  return GRPC.success({multiplayer = DCS.isMultiplayer()})
end

GRPC.methods.isServer = function()
  return GRPC.success({server = DCS.isServer()})
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