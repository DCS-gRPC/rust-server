--
-- APIs for functions that are not built-in to the DCS Mission Scripting Environment
--

GRPC.methods.requestMissionAssignment = function(params)
    return GRPC.errorUnimplemented("This method is not implemented")
end

GRPC.methods.joinMission = function(params)
    return GRPC.errorUnimplemented("This method is not implemented")
end

GRPC.methods.eval = function(params)
    if GRPC.evalEnabled ~= true then
        return GRPC.errorPermissionDenied("eval operation is disabled")
    end

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
