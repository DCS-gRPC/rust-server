
# Installer of DCS-gRPC

## To Build
`#pyinstaller --clean --onefile  -y -n "DCS-gRPC-Installer" .\main.py`

## What

1. It collects all releases from the Github api.
2. Makes the user select the version (latest probably 99% percent of the time), but fallback possibilities might be nice. 
3. Downloads the .zip artifact and unzips in install location (selected by user)
4. Updates the missionscripting.lua file. 
   - MissionScripting.lua needs to be selected. No other lua file will be accepted as DCS only reads that file.
   - `dofile(lfs.writedir()..[[Scripts\DCS-gRPC\grpc-mission.lua]])` is on the first line below `dofile('Scripts/ScriptingSystem.lua')`
   - all other `dofile(lfs.writedir()..[[Scripts\DCS-gRPC\grpc-mission.lua]])` or similar (grpc-mission.lua) will be removed so in case of reinstall any possible error will be reset
