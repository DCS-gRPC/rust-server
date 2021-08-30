env.info("[GRPC] loading ...")

--
-- load and start RPC
--
package.loaded["dcs_grpc_server"] = nil
grpc = require "dcs_grpc_server"
grpc.start()
GRPC.stopped = false
GRPC.options = {
  evalEnabled = false
}

--
-- Methods to set options
--

GRPC.enableEval = function()
  GRPC.options.evalEnabled = true
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
-- RPC methods
--

GRPC.methods = {}
dofile(GRPC.basePath .. [[methods\atmosphere.lua]])
dofile(GRPC.basePath .. [[methods\coalitions.lua]])
dofile(GRPC.basePath .. [[methods\custom.lua]])
dofile(GRPC.basePath .. [[methods\group.lua]])
dofile(GRPC.basePath .. [[methods\mission.lua]])
dofile(GRPC.basePath .. [[methods\trigger.lua]])
dofile(GRPC.basePath .. [[methods\unit.lua]])
dofile(GRPC.basePath .. [[methods\world.lua]])

--
-- RPC request handler
--
local function handleRequest(method, params)
  local fn = GRPC.methods[method]

  if type(fn) == "function" then
    local ok, result = pcall(fn, params)
    if ok then
      return result
    else
      env.error("[GRPC] error executing "..method..": "..tostring(result))
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

--
-- execute gRPC requests every ~0.02 seconds
--
local function next()
  local i = 0
  while grpc.next(handleRequest) do
    i = i + 1
    if i > 10 then
      break
    end
  end
end

timer.scheduleFunction(function()
  if not GRPC.stopped then
    local ok, err = pcall(next)
    if not ok then
      env.error("[GRPC] Error retrieving next command: "..tostring(err))
    end

    return timer.getTime() + .02 -- return time of next call
  end
end, nil, timer.getTime() + .02)

GRPC.toLatLonPosition = function(pos)
  local lat, lon, alt = coord.LOtoLL(pos)
  return {
    lat = lat,
    lon = lon,
    alt = alt,
  }
end

local eventHandler = {}
function eventHandler:onEvent(event)
  if not GRPC.stopped then
    local ok, err = pcall(GRPC.onDcsEvent, event)
    if not ok then
      env.error("[GRPC] Error in event handler: "..tostring(err))
    end
  end
end
world.addEventHandler(eventHandler)

env.info("[GRPC] loaded ...")
