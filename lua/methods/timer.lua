--
-- RPC timer functions
-- https://wiki.hoggitworld.com/view/DCS_singleton_timer
--

-- https://wiki.hoggitworld.com/view/DCS_func_getTime
GRPC.methods.getTime = function(params)
  return GRPC.success(
    {
      time = timer.getTime() 
    }
  )
end

-- https://wiki.hoggitworld.com/view/DCS_func_getAbsTime
GRPC.methods.getAbsoluteTime = function(params)
  return GRPC.success(
    {
      time = timer.getAbsTime() 
    }
  )
end

-- https://wiki.hoggitworld.com/view/DCS_func_getTime0
GRPC.methods.getTimeZero = function(params)
  return GRPC.success(
    {
      time = timer.getTime0() 
    }
  )
end