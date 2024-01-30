import PySimpleGUI as sg
import os
from install import Install
import requests
import io
from PIL import Image
import base64

width = 800
heigth = 500

sg.theme('DarkTeal2')

DEF_BUTTON_COLOR = ('white', 'black')
bluebutton='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'
def GraphicButton(text, key, disabled, image_data, color=DEF_BUTTON_COLOR, size=(100, 50)):
    return sg.Button(text, disabled=disabled, image_data=resize_base64_image(image_data, size), button_color=color, font='Any 15', pad=(0, 0), key=key, border_width=0)

def resize_base64_image(image64, size):
    image_file = io.BytesIO(base64.b64decode(image64))
    img = Image.open(image_file)
    img.thumbnail(size,  Image.LANCZOS)
    bio = io.BytesIO()
    img.save(bio, format='PNG')
    imgbytes = bio.getvalue()
    return imgbytes

versionsRetrieved=False
selectedVersion = None
versions = {}
versionLayout = []
def GetandFillVersion():
    response = requests.get("https://api.github.com/repos/DCS-gRPC/rust-server/releases")

    if not 200 == response.status_code:
        return False, "Could not load versions, restart the installer to try again"

    versionResponse = response.json()

    global versionLayout
    global versions
    global selectedVersion
    for version in versionResponse[0:5]: #Show 6 latest versions
        for asset in version['assets']:
            assetName = asset['name']
            if assetName.endswith('.zip'):
                versions[version['tag_name']] = asset['browser_download_url']
                if selectedVersion == None:
                    selectedVersion = version['tag_name']
                    button = sg.Radio(f"{version['tag_name']} (latest)" , "version", key=f"vRadio {version['tag_name']}", enable_events=True, default=True)
                else:
                    button = sg.Radio(version['tag_name'], "version", key=f"vRadio {version['tag_name']}", enable_events=True)
                versionLayout += [[button]]
    
GetandFillVersion()

def DefineLayout():
    start_layout = [
        [sg.Text("Welcome to the DCS-gRPC installer")], 
    ]

    version_selection_layout = [
        [sg.Frame(layout=versionLayout, title="Select Version", key="Version Selector", size=(width-50,heigth-50))]
    ]

    file_selection_layout = [
        [sg.Text("Select Installation Locations")],
        [sg.HSeparator()],
        [sg.Text("DCS Saved Games Folder")],
        [   
            sg.Input(key="INSTALL_DIR", enable_events=True), 
            sg.FolderBrowse(key="INSTALL_DIR_BROWSE")
        ],
        [sg.HSeparator()],
        [sg.Text("DCS MissionScripting.lua")],
        [
            sg.Input(key="MISSIONFILE_LOC", enable_events=True), 
            sg.FileBrowse(key="MISSIONFILE_LOC_BROWSE", file_types=(("Lua Files", ".lua"),))
        ]
    ]

    overview_layout = [
        [sg.Text("Final checks")], 
        [sg.Text("Click install to continue")],
        [sg.HSeparator()],
        [sg.Text("Version")],
        [sg.Text(f"{selectedVersion}", key="OVERVIEW_VERSION")],
        [sg.HSeparator()],
        [sg.Text("Installation Directory")],
        [sg.Text("Not Set", key="OVERVIEW_INSTALLDIR")],
        [sg.HSeparator()],
        [sg.Text("MissionScripting.lua")],
        [sg.Text("Not Set", key="OVERVIEW_SCRIPTFILE")]
    ]

    columns = [ #PAGES
        [
            sg.Column(start_layout, key='START'),
            sg.Column(version_selection_layout, key='VERSION', visible=False),
            sg.Column(file_selection_layout, key='FILE_SELECTION', visible=False),
            sg.Column(overview_layout, key='OVERVIEW', visible=False)
        ]
    ]

    layout = [
        [
            sg.Frame(layout=columns, title="DCS-gRPC Installer", size=(width-50,heigth-50))
        ],
        [sg.HSeparator()],
        [
            [
                GraphicButton("Previous", "Previous", True, bluebutton),
                GraphicButton("Next", "Next", False, bluebutton),
                GraphicButton("Install", "Install", True, bluebutton)
            ]
        ]
    ]
    return layout

window = sg.Window(
    "Install DCS-gRPC", 
    DefineLayout(),
    size = (width,heigth)
    )

pages = [ window["START"], window["VERSION"], window["FILE_SELECTION"], window["OVERVIEW"] ]
pageIndex = 0

def ValidateAndInstall():

    install_path = window["INSTALL_DIR"].get()
    if not os.path.exists(install_path):
        return False, "DCS Saved games folder not found"

    scriptingFile = window["MISSIONFILE_LOC"].get()
    if not os.path.exists(scriptingFile):
        return False, "MissionScripting.lua file not found"

    if not scriptingFile.lower().endswith("missionscripting.lua"):
        return False, "your file was not the expected MissionScripting.lua file"

    success, message = Install(install_path,scriptingFile, versions[selectedVersion])

    return success, message

def ButtonClicked(event, values):
 
    if event == sg.WIN_CLOSED:
        return False

    window["Previous"].update(disabled=False)
    window["Next"].update(disabled=False)
    window["Install"].update(disabled=True)

    global pageIndex
    if(event == "Next"):
        if(len(pages) -1 > pageIndex):
            pages[pageIndex].update(visible=False)
            pages[pageIndex+1].update(visible=True)
            pageIndex+= 1
    
        if pages[pageIndex].key == "OVERVIEW":
            window["Next"].update(disabled=True)
            window["Install"].update(disabled=False)

    if(event == "Previous"):
        if(0 < pageIndex):
            pages[pageIndex].update(visible=False)
            pages[pageIndex-1].update(visible=True)
            pageIndex-=1
        
        if pages[pageIndex].key == "START":
            window["Previous"].update(disabled=True)

    if(event == "Install"):
        success, message = ValidateAndInstall()
        if success: 
            sg.popup(f"Everything installed succesfully")
            return False
        else:
            sg.popup(f"Not all fields were correct: {message}")

    if(event == "MISSIONFILE_LOC"):
        window["OVERVIEW_SCRIPTFILE"].update(window["MISSIONFILE_LOC"].get())
    
    if(event == "INSTALL_DIR"):
        window["OVERVIEW_INSTALLDIR"].update(window["INSTALL_DIR"].get())

    return True

# Create an event loop
while True:
    event, values = window.read()
    if event == sg.WIN_CLOSED:
        break

    if event.startswith('vRadio'):
        version = event.split(' ')[1]
        selectedVersion = version
        window["OVERVIEW_VERSION"].update(version)
    else:
        success = ButtonClicked(event, values)
    
    if not success:
        break

window.close()

#pyinstaller --clean --onefile  -y -n "install-dcs-gRPC" --add-data="files\DCS-gRPC-0.7.1.zip;files" .\main.py
