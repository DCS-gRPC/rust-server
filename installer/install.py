import os
import sys
import shutil
import zipfile
import urllib.request


def __UpdateMissionScriptingLua(path):
    gRPCstartLine = "dofile(lfs.writedir()..[[Scripts\DCS-gRPC\grpc-mission.lua]])"

    #Supports replacing grpc-mission.lua to another file if the needs arise later and searching for the new file at the same time
    searchStrings =  ["grpc-mission.lua"] 
    searchStringSystem = "scripts/scriptingSystem.lua"

    if not os.path.exists(path):
        return False

    newlines = []
    with open(path, 'r') as f:
        lines = f.readlines()
        i = 0

        systemLine = -1
        for line in lines:
            if any(searchString in line for searchString in searchStrings):
                newlines += "\n" #ADD EMPTY LINE 
                line = "\n" #REMOVE LINE

            if systemLine == -1 and searchStringSystem.lower() in line.lower():
                systemLine = i

            newlines += [line]
            if systemLine == i:
                newlines += gRPCstartLine
            i+=1
            
    with open(path, 'w') as f:
        f.writelines(newlines)

    return True

def __InstallInFiles(installDir, downloadUrl):
    zipPath = "./dcs-gRPC.zip"
    urllib.request.urlretrieve(downloadUrl, zipPath)
    with zipfile.ZipFile(zipPath, 'r') as zip_ref:
        zip_ref.extractall(installDir)

    os.remove(zipPath)
    return True

def Install(installDir, missionScriptingFile, assetDownload):
    if not os.path.exists(installDir):
        return False, "DCS Saved games folder not found"

    if not os.path.exists(missionScriptingFile):
        return False, "MissionScripting.lua file not found"

    if not missionScriptingFile.lower().endswith("missionscripting.lua"):
        return False, "your file was not the expected MissionScripting.lua file"

    success = __UpdateMissionScriptingLua(missionScriptingFile)
    if not success: 
        return False, "Something went wrong while update the MissionScripting.lua file"

    success = __InstallInFiles(installDir, assetDownload)
    if not success: 
        return False, "Something went wrong while installing all the Saved Games files"


    return True, "Installed Succesfully"