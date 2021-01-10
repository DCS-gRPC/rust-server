env.info("[GRPC] loading ...")

--
-- load and start RPC
--
package.loaded["dcs_grpc_server"] = nil
grpc = require "dcs_grpc_server"
grpc.start()
local stopped = false

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
    error = msg
  }
end

--
-- RPC methods
--

GRPC.methods = {}
dofile(GRPC.basePath .. [[methods\trigger.lua]])

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
  if not stopped then
    local ok, err = pcall(next)
    if not ok then
      env.error("[GRPC] Error retrieving next command: "..tostring(err))
    end

    return timer.getTime() + .02 -- return time of next call
  end
end, nil, timer.getTime() + .02)

--
-- listen to DCS events
--
local function identifier(obj)
  if obj == nil then
    return nil
  end
  return obj:getName()
end

local function onEvent(event)
  if event.id == world.event.S_EVENT_MISSION_END then
    grpc.stop()
    stopped = true
  end
end

local eventHandler = {}
function eventHandler:onEvent(event)
  if not stopped then
    local ok, err = pcall(onEvent, event)
    if not ok then
      env.error("[GRPC] Error in event handler: "..tostring(err))
    end
  end
end
world.addEventHandler(eventHandler)

env.info("[GRPC] loaded ...")
