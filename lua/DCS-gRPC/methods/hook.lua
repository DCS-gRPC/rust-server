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
