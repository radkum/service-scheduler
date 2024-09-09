FOR /F "tokens=1" %%F IN ('cd') DO SET CURR_DIR=%%F
sc create sample binPath=%CURR_DIR%\target\debug\win-service.exe