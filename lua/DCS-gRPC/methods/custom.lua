--
-- APIs for functions that are not built-in to the DCS Mission Scripting Environment
--

GRPC.methods.requestMissionAssignment = function()
    return GRPC.errorUnimplemented("This method is not implemented")
end

GRPC.methods.joinMission = function()
    return GRPC.errorUnimplemented("This method is not implemented")
end

GRPC.methods.abortMission = function()
    return GRPC.errorUnimplemented("This method is not implemented")
end

GRPC.methods.getMissionStatus = function()
    return GRPC.errorUnimplemented("This method is not implemented")
end

GRPC.methods.missionEval = function(params)
    local fn, err = loadstring(params.lua)
    if not fn then
        return GRPC.error("Failed to load Lua code: "..err)
    end

    local ok, result = pcall(fn)
    if not ok then
        return GRPC.error("Failed to execute Lua code: "..result)
    end

    return GRPC.success(net.lua2json(result))
end
