Name "pylon"

RequestExecutionLevel User

OutFile "pylon-cli-windows.exe"
InstallDir $PROFILE\.pylon

Section

    CreateDirectory $INSTDIR\bin
    SetOutPath $INSTDIR\bin
    File target\release\pylon-cli.exe
    Rename $INSTDIR\bin\pylon-cli.exe $INSTDIR\bin\pylon.exe

    EnVar::AddValue "PATH" "$INSTDIR\bin"
    Pop $0

    SetOutPath $INSTDIR
    WriteUninstaller $INSTDIR\uninstall.exe

    MessageBox MB_OK "Success! Installed to: $INSTDIR$\n$\nTo get started, restart your terminal and \
        run the following command:$\n$\n    pylon --help$\n$\nTo uninstall run: $INSTDIR\uninstall.exe"

SectionEnd

Section "Uninstall"

    EnVar::DeleteValue "PATH" "$INSTDIR\bin"
    Pop $0

    Delete $INSTDIR\uninstall.exe
    Delete $INSTDIR\bin\pylon.exe
    RMDir $INSTDIR\bin
    RMDir $INSTDIR

    RMDir /r $LOCALAPPDATA\Pylon

SectionEnd
