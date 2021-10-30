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
  local result = {}
  for _, c in pairs(coalition.side) do
    if params.coalition == nil or params.coalition == c then
      -- https://wiki.hoggitworld.com/view/DCS_func_getGroups
      local groups = coalition.getGroups(c, params.category)

      for _, group in ipairs(groups) do
        table.insert(result, GRPC.exporters.group(group))
      end
    end
  end

  return GRPC.success({groups = result})
end

GRPC.methods.getMainReferencePoint = function(params)
  local referencePoint = coalition.getMainRefPoint(params.coalition)

  return GRPC.success({
    position = GRPC.toLatLonPosition(referencePoint)
  })
end