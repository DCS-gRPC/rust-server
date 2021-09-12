--
-- Hook actions
-- Docs: /DCS World/API/DCS_ControlAPI.html
--

local GRPC = GRPC

GRPC.methods.getMissionName = function()
  return GRPC.success({name = DCS.getMissionName()})
end
