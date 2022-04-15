if not GRPC then
  GRPC = {}
end

-- load settings from `Saved Games/DCS/Config/dcs-grpc.lua`
do
  env.info("[GRPC] Checking optional config at `Config/dcs-grpc.lua` ...")
  local file, err = io.open(lfs.writedir() .. [[Config\dcs-grpc.lua]], "r")
  if file then
    local f = assert(loadstring(file:read("*all")))
    setfenv(f, GRPC)
    f()
    env.info("[GRPC] `Config/dcs-grpc.lua` successfully read")
  else
    env.info("[GRPC] `Config/dcs-grpc.lua` not found (" .. tostring(err) .. ")")
  end
end

-- Set default settings.
if not GRPC.luaPath then
  GRPC.luaPath = lfs.writedir() .. [[Scripts\DCS-gRPC\]]
end
if not GRPC.dllPath then
  GRPC.dllPath = lfs.writedir() .. [[Mods\tech\DCS-gRPC\]]
end
if GRPC.throughputLimit == nil or GRPC.throughputLimit == 0 or not type(GRPC.throughputLimit) == "number" then
  GRPC.throughputLimit = 600
end

-- Let DCS know where to find the DLLs
if not string.find(package.cpath, GRPC.dllPath) then
  package.cpath = package.cpath .. [[;]] .. GRPC.dllPath .. [[?.dll;]]
end

-- Load DLL before `require` gets sanitized.
local ok, grpc = pcall(require, "dcs_grpc_hot_reload")
if ok then
  env.info("[GRPC] loaded hot reload version")
else
  grpc = require("dcs_grpc")
end

-- Keep a reference to `lfs` before it gets sanitized
local lfs = _G.lfs

local loaded = false
function GRPC.load()
  if loaded then
    env.info("[GRPC] already loaded")
    return
  end

  local env = setmetatable({grpc = grpc, lfs = lfs}, {__index = _G})
  local f = setfenv(assert(loadfile(GRPC.luaPath .. [[grpc.lua]])), env)
  f()

  loaded = true
end

if GRPC.autostart == true then
  env.info("[GRPC] auto starting")
  GRPC.load()
end
