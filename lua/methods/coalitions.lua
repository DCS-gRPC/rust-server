--
-- RPC unit actions
-- https://wiki.hoggitworld.com/view/DCS_singleton_coalition
--

local GRPC = GRPC
local coalition = coalition

GRPC.methods.getPlayers = function(params)
  local units = coalition.getPlayers(params.coalition)

  result = {}

  for i, unit in ipairs(units) do
    result[i] = GRPC.exporters.unit(unit)
  end
  return GRPC.success({units = result})
end

GRPC.methods.getGroups = function(params)
  -- https://wiki.hoggitworld.com/view/DCS_func_getGroups
  local groups = coalition.getGroups(params.coalition, params.category)

  local result = {}
  for i, group in ipairs(groups) do
    result[i] = GRPC.exporters.group(group)
  end

  return GRPC.success({groups = result})
end
