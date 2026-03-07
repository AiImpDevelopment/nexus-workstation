; NEXUS NSIS Installer Configuration
; Used by tauri.conf.json nsis settings

!define APP_NAME "NEXUS AI Workstation"
!define APP_PUBLISHER "TiKcoc"
!define APP_URL "https://github.com/TiKcoc/nexus-workstation"
!define APP_DESCRIPTION "Your complete AI stack. One desktop app."

; Installation directory
InstallDir "$LOCALAPPDATA\${APP_NAME}"

; Request admin privileges for Windows 11
RequestExecutionLevel admin
