@echo off
cd ..
echo COMMENCING BUILD AND RUNNING EXE
echo.
@echo on

rem NOTE: This command also builds an .rlib for the
rem file defined in Cargo.toml's [lib] section, which
rem is unnecessary for the standalone exe that gets
rem built. Obviously not a big problem, but still an
rem odd sideeffect.
cargo run --bin trustines

@echo off
echo.
echo BUILD AND RUN FINISHED
echo.
pause
