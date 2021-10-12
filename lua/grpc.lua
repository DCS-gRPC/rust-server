local isMissionEnv = _G.DCS == nil

if isMissionEnv then
  env.info("[GRPC] mission loading ...")
end

--
-- set default settings
--

if _G.GRPC == nil then
  GRPC = {}
end

if GRPC.basePath == nil then
  GRPC.basePath = lfs.writedir()..[[Scripts\DCS-gRPC\]]
end
if GRPC.evalEnabled == nil then
  GRPC.evalEnabled = false
end
if GRPC.host == nil then
  GRPC.host = "127.0.0.1"
end
if GRPC.port == nil then
  GRPC.port = 50051
end
if GRPC.debug == true then
  GRPC.debug = true
end

--
-- load and start RPC
--

local ok, grpc = pcall(require, "dcs_grpc_server_hot_reload")
if ok then
  if isMissionEnv then
    env.info("[GRPC] loaded hot reload version")
  else
    log.write("[GRPC-Hook]", log.INFO, "loaded hot reload version")
  end
else
  grpc = require("dcs_grpc_server")
end

if isMissionEnv then
  grpc.start(GRPC.host, GRPC.port, GRPC.debug)
end

--
-- Export methods
--

GRPC.exporters = {}
dofile(GRPC.basePath .. [[exporters\object.lua]])

--
-- Helper methods
--

GRPC.success = function(result)
  return {
    result = result
  }
end

GRPC.error = function(msg)
  return {
    error = {
      message = msg,
    }
  }
end

GRPC.onChatMessage = function(playerID, msg, all)
  return grpc.on_chat_message(playerID, msg, all)
end

--
-- Logging methods
--

GRPC.logError = function(err)
  grpc.log_error(err)
  env.error("[GRPC] "..err)
end

GRPC.logWarning = function(err)
  grpc.log_warning(err)
  env.warning("[GRPC] "..err)
end

--- The client specified an invalid argument
GRPC.errorInvalidArgument = function(msg)
  return {
    error = {
      type = "INVALID_ARGUMENT",
      message = msg,
    }
  }
end

--- Some requested entity was not found.
GRPC.errorNotFound = function(msg)
  return {
    error = {
      type = "NOT_FOUND",
      message = msg,
    }
  }
end

--- The entity that a client attempted to create already exists.
GRPC.errorAlreadyExists = function(msg)
  return {
    error = {
      type = "ALREADY_EXISTS",
      message = msg,
    }
  }
end

--- The operation is not implemented or is not supported/enabled in this service.
GRPC.errorUnimplemented = function(msg)
  return {
    error = {
      type = "UNIMPLEMENTED",
      message = msg,
    }
  }
end

--- The caller does not have permission to execute the specified operation.
GRPC.errorPermissionDenied = function(msg)
  return {
    error = {
      type = "PERMISSION_DENIED",
      message = msg,
    }
  }
end

--
-- Helper methods
--

GRPC.toLatLonPosition = function(pos)
  local lat, lon, alt = coord.LOtoLL(pos)
  return {
    lat = lat,
    lon = lon,
    alt = alt,
  }
end

--
-- RPC methods
--

GRPC.methods = {}
dofile(GRPC.basePath .. [[methods\atmosphere.lua]])
dofile(GRPC.basePath .. [[methods\coalitions.lua]])
dofile(GRPC.basePath .. [[methods\controllers.lua]])
dofile(GRPC.basePath .. [[methods\custom.lua]])
dofile(GRPC.basePath .. [[methods\group.lua]])
dofile(GRPC.basePath .. [[methods\hook.lua]])
dofile(GRPC.basePath .. [[methods\mission.lua]])
dofile(GRPC.basePath .. [[methods\timer.lua]])
dofile(GRPC.basePath .. [[methods\trigger.lua]])
dofile(GRPC.basePath .. [[methods\unit.lua]])
dofile(GRPC.basePath .. [[methods\world.lua]])

--
-- RPC request handler
--

local stopped = false
GRPC.stop = function()
  grpc.stop()
  stopped = true
end

local function handleRequest(method, params)
  local fn = GRPC.methods[method]

  if type(fn) == "function" then
    local ok, result = xpcall(function() return fn(params) end, debug.traceback)
    if ok then
      return result
    else
      GRPC.logError("error executing "..method..": "..tostring(result))
      return {
        error = tostring(result)
      }
    end
  else
    return {
      error = "unsupported method "..method
    }
  end
end

local MISSION_ENV = 1
local HOOK_ENV = 2

if isMissionEnv then
  -- execute gRPC requests every ~0.02 seconds
  local function next()
    local i = 0
    while grpc.next(MISSION_ENV, handleRequest) do
      i = i + 1
      if i > 10 then
        break
      end
    end
  end

  timer.scheduleFunction(function()
    if not stopped then
      local ok, err = pcall(next)
      if not ok then
        GRPC.logError("Error retrieving next command: "..tostring(err))
      end

      return timer.getTime() + .02 -- return time of next call
    end
  end, nil, timer.getTime() + .02)

  local eventHandler = {}
  function eventHandler:onEvent(event)
    if not stopped then
      local ok, result = xpcall(function() return GRPC.onDcsEvent(event) end, debug.traceback)
      if ok then
        if result ~= nil then
          grpc.event(result)
          if result.event.type == "missionEnd" then
            GRPC.stop()
          end
        end
      else
        GRPC.logError("Error in event handler: "..tostring(err))
      end
    end
  end
  world.addEventHandler(eventHandler)
else -- hook env
  -- execute gRPC requests every 10th simulation frame
  local function next()
    local i = 0
    while grpc.next(HOOK_ENV, handleRequest) do
      i = i + 1
      if i > 10 then
        break
      end
    end
  end

  local frame = 0
  function GRPC.onSimulationFrame()
    frame = frame + 1
    if frame >= 10 then
      frame = 0
      local ok, err = pcall(next)
      if not ok then
        log.write("[GRPC]", log.ERROR, "Error retrieving next command: "..tostring(err))
      end
    end
  end
end

if isMissionEnv then
  env.info("[GRPC] loaded ...")
end
