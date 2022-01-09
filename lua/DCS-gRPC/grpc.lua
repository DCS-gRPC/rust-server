local isMissionEnv = DCS == nil

if isMissionEnv then
  env.info("[GRPC] mission loading ...")
end

--
-- load and start RPC
--

if isMissionEnv then
  grpc.start({
    writeDir = lfs.writedir(),
    dllPath = GRPC.dllPath,
    host = GRPC.host,
    port = GRPC.port,
    debug = GRPC.debug,
    evalEnabled = GRPC.evalEnabled
  })
end

--
-- Export methods
--

GRPC.exporters = {}
dofile(GRPC.luaPath .. [[exporters\object.lua]])

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

--
-- Logging methods
--

GRPC.logError = function(msg)
  grpc.logError(msg)

  if isMissionEnv then
    env.error("[GRPC] "..msg)
  else
    log.write("[GRPC-Hook]", log.ERROR, msg)
  end
end

GRPC.logWarning = function(msg)
  grpc.logWarning(msg)

  if isMissionEnv then
    env.info("[GRPC] "..msg)
  else
    log.write("[GRPC-Hook]", log.WARNING, msg)
  end
end

GRPC.logInfo = function(msg)
  grpc.logInfo(msg)
  if isMissionEnv then
    env.info("[GRPC] "..msg)
  else
    log.write("[GRPC-Hook]", log.INFO, msg)
  end
end

GRPC.logDebug = function(msg)
  grpc.logDebug(msg)
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

GRPC.event = grpc.event
--
-- RPC methods
--

GRPC.methods = {}
dofile(GRPC.luaPath .. [[methods\atmosphere.lua]])
dofile(GRPC.luaPath .. [[methods\coalitions.lua]])
dofile(GRPC.luaPath .. [[methods\controllers.lua]])
dofile(GRPC.luaPath .. [[methods\custom.lua]])
dofile(GRPC.luaPath .. [[methods\group.lua]])
dofile(GRPC.luaPath .. [[methods\hook.lua]])
dofile(GRPC.luaPath .. [[methods\mission.lua]])
dofile(GRPC.luaPath .. [[methods\net.lua]])
dofile(GRPC.luaPath .. [[methods\timer.lua]])
dofile(GRPC.luaPath .. [[methods\trigger.lua]])
dofile(GRPC.luaPath .. [[methods\unit.lua]])
dofile(GRPC.luaPath .. [[methods\world.lua]])

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

-- Adjust the interval at which the gRPC server is polled for requests based on the throughput
-- limit. The higher the throughput limit, the more often the gRPC is polled per second.
local interval = math.max(0.03, math.min(1.0, 16 / GRPC.throughputLimit))
local callsPerTick = math.ceil(GRPC.throughputLimit * interval)

if isMissionEnv then
  GRPC.logInfo(
    "Limit request execution at max. " .. tostring(callsPerTick) .. " calls every " ..
    tostring(interval) .. "s (≙ throughput of " .. tostring(GRPC.throughputLimit) .. ")"
  )

  -- execute gRPC requests
  local function next()
    local i = 0
    while grpc.next(MISSION_ENV, handleRequest) do
      i = i + 1
      if i >= callsPerTick then
        break
      end
    end
  end

  -- scheduel gRPC request execution
  timer.scheduleFunction(function()
    if not stopped then
      local ok, err = pcall(next)
      if not ok then
        GRPC.logError("Error retrieving next command: "..tostring(err))
      end

      return timer.getTime() + interval -- return time of next call
    end
  end, nil, timer.getTime() + interval)

  -- listen for events
  local eventHandler = {}
  function eventHandler:onEvent(event)
    local _ = self -- make linter happy

    if not stopped then
      local ok, result, err = xpcall(function() return GRPC.onDcsEvent(event) end, debug.traceback)
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
  -- execute gRPC requests
  local function next()
    local i = 0
    while grpc.next(HOOK_ENV, handleRequest) do
      i = i + 1
      if i > callsPerTick then
        break
      end
    end
  end

  -- scheduel gRPC request execution
  local skipFrames = math.ceil(interval / 0.016) -- 0.016 = 16ms = 1 frame at 60fps
  local frame = 0
  function GRPC.onSimulationFrame()
    frame = frame + 1
    if frame >= skipFrames then
      frame = 0
      local ok, err = pcall(next)
      if not ok then
        GRPC.logError("Error retrieving next command: "..tostring(err))
      end
    end
  end
end

if isMissionEnv then
  env.info("[GRPC] loaded ...")
end
