@echo off
cargo build --release
if %ERRORLEVEL% NEQ 0 exit /b %ERRORLEVEL%

if exist dist rmdir /s /q dist
mkdir dist
copy target\release\Disc-Golf-Man.exe dist\
xcopy assets dist\assets\ /s /e /q

echo.
echo Build complete: dist\Disc-Golf-Man.exe
