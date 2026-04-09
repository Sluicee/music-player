!include nsDialogs.nsh
!include LogicLib.nsh

Var CheckboxDeleteData

# ── POST-INSTALL HOOK ──────────────────────────────────────────────────────────
# Move the sidecar(s) and other binaries to a 'bin' folder for a cleaner layout.
!macro NSIS_HOOK_POSTINSTALL
    DetailPrint "Organizing files..."
    CreateDirectory "$INSTDIR\bin"
    
    # Move ffmpeg sidecar. Tauri names it ffmpeg-<triple>.exe
    # We use a wildcard to be triple-agnostic.
    FindFirst $0 $1 "$INSTDIR\ffmpeg-*.exe"
    StrCmp $0 "" no_ffmpeg
    Rename "$INSTDIR\$1" "$INSTDIR\bin\ffmpeg.exe"
    FindClose $0
    
    no_ffmpeg:
!macroend

# ── UNINSTALLER CUSTOMIZATION ──────────────────────────────────────────────────

# This macro is called by the Tauri template to allow uninstaller customization
!macro NSIS_HOOK_UNINSTALL
    # Define the show function for the confirmation page
    !define MUI_CUSTOMFUNCTION_UNPAGE_CONFIRM_SHOW un.AddDeleteDataCheckbox
!macroend

Function un.AddDeleteDataCheckbox
    # Create the checkbox on the existing confirmation page
    # Positioned at the bottom
    ${NSD_CreateCheckbox} 0 -20u 100% 10u "Удалить данные приложения (кэш и настройки)?"
    Pop $CheckboxDeleteData
    # Set default to unchecked for safety
    ${NSD_Uncheck} $CheckboxDeleteData
FunctionEnd

# ── POST-UNINSTALL HOOK ────────────────────────────────────────────────────────
# Clean up app data if the checkbox was checked.
!macro NSIS_HOOK_POSTUNINSTALL
    ${NSD_GetState} $CheckboxDeleteData $0
    ${If} $0 == ${BST_CHECKED}
        DetailPrint "Deleting application data..."
        # App data dir is %LOCALAPPDATA%\com.sluic.musicplayer
        # SetShellVarContext current ensures we are in the user context
        SetShellVarContext current
        RMDir /r "$LOCALAPPDATA\com.sluic.musicplayer"
    ${EndIf}
!macroend
