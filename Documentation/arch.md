 For development:
Apply to arch.md
Run
This will:
Build the frontend
Start the Tauri development server
Open the app window
For release build:
Apply to arch.md
Run
This will:
Build the frontend
Create the MSI installer and EXE
Show you where to find the output files
To clean build artifacts:
Apply to arch.md
Run
This will remove all build artifacts if you need to start fresh.
The changes I made:
Fixed the WiX configuration in tauri.conf.json to remove invalid options
Updated the build script to properly handle both development and release modes
Added a clean script to help with troubleshooting
Made the development mode actually open the app window
Added proper error handlings