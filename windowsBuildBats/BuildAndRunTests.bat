@echo off
cd ..
echo COMMENCING BUILD AND RUNNING TESTS
echo.
@echo on

cargo test

@echo off
echo.
echo BUILD AND TESTS FINISHED
echo.
pause
