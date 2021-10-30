--
-- RPC timer functions
-- https://wiki.hoggitworld.com/view/DCS_singleton_timer
--

-- https://wiki.hoggitworld.com/view/DCS_func_getTime
GRPC.methods.getTime = function()
  return GRPC.success(
    {
      time = timer.getTime()
    }
  )
end

-- https://wiki.hoggitworld.com/view/DCS_func_getAbsTime
GRPC.methods.getAbsoluteTime = function()
  return GRPC.success(
    {
      time = timer.getAbsTime(),
      day = env.mission.date.Day,
      month = env.mission.date.Month,
      year = env.mission.date.Year,
    }
  )
end

-- https://wiki.hoggitworld.com/view/DCS_func_getTime0
GRPC.methods.getTimeZero = function()
  return GRPC.success(
    {
      time = timer.getTime0(),
      day = env.mission.date.Day,
      month = env.mission.date.Month,
      year = env.mission.date.Year,
    }
  )
end