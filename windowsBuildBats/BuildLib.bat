@echo off
cd ..
echo COMMENCING LIB BUILD
echo.
@echo on

cargo build --lib

@echo off
echo.
echo LIB BUILD FINISHED
echo.
pause
