--
-- RPC unit actions
-- https://wiki.hoggitworld.com/view/DCS_singleton_coalition
--

local GRPC = GRPC
local coalition = coalition

GRPC.methods.getPlayers = function(params)
  local result = {}
  for _, c in pairs(coalition.side) do
    if params.coalition == 0 or params.coalition - 1 == c then -- Decrement for non zero-indexed gRPC enum
      -- https://wiki.hoggitworld.com/view/DCS_func_getPlayers
      local units = coalition.getPlayers(c)

      for _, unit in ipairs(units) do
        table.insert(result, GRPC.exporters.unit(unit))
      end
    end
  end
  return GRPC.success({units = result})
end

GRPC.methods.getGroups = function(params)
  local result = {}
  for _, c in pairs(coalition.side) do
    if params.coalition == 0 or params.coalition - 1 == c then -- Decrement for non zero-indexed gRPC enum
      -- https://wiki.hoggitworld.com/view/DCS_func_getGroups
      local groups = coalition.getGroups(c, params.category)

      for _, group in ipairs(groups) do
        table.insert(result, GRPC.exporters.group(group))
      end
    end
  end

  return GRPC.success({groups = result})
end

-- This method should be called once per coalition per mission so using COALITION_ALL to save 2
-- API calls is not worth the extra code.
GRPC.methods.getMainReferencePoint = function(params)
  if params.coalition == 0 then
    return GRPC.errorInvalidArgument("a specific coalition must be chosen")
  end

  local referencePoint = coalition.getMainRefPoint(params.coalition - 1) -- Decrement for non zero-indexed gRPC enum

  return GRPC.success({
    position = GRPC.toLatLonPosition(referencePoint)
  })
end