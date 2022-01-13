--
-- RPC net actions
-- https://wiki.hoggitworld.com/view/DCS_singleton_net
--

GRPC.methods.sendChatTo = function(params)
    -- note: it was explicitly decided not to place "from player id" parameter
    --       due to the magnitude of a social attack vector.
    --       https://github.com/DCS-gRPC/rust-server/pull/94#discussion_r780777794
    net.send_chat_to(params.message, params.targetPlayerId)
    return GRPC.success(nil)
end

GRPC.methods.sendChat = function(params)
    if params.coalition > 1 then
        return GRPC.errorInvalidArgument("Chat messages can only be sent to all or neutral/spectators")
    end

    local toAll = params.coalition ~= 1
    net.send_chat(params.message, toAll)
    return GRPC.success(nil)
end

GRPC.methods.getPlayerInfo = function(params)
    local playerInfo = net.get_player_info(params.id);

    if playerInfo == nil then
        return GRPC.errorNotFound("requested player could not be found")
    end

    local normalizedResult = {
        ["id"] = playerInfo.id,
        ["name"] = playerInfo.name,
        ["coalition"] = playerInfo.side + 1, -- common.Coalition enum offset
        ["slot"] = playerInfo.slot,
        ["ping"] = playerInfo.ping,
        ["remoteAddress"] = playerInfo.ipaddr,
        ["ucid"] = playerInfo.ucid,
        ["locale"] = playerInfo.lang
    }
    return GRPC.success(normalizedResult)
end
