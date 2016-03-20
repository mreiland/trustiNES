@echo off
cd ..
echo COMMENCING BUILD AND RUNNING EXE
echo.

rem NOTE: This command also builds an .rlib for the
rem file defined in Cargo.toml's [lib] section, which
rem is unnecessary for the standalone exe that gets
rem built. Obviously not a big problem, but still an
rem odd sideeffect.

@echo on
cargo build --bin trustines

@echo off
echo.
echo BUILD FINISHED
echo.

if exist target\debug\trustines.exe (
  echo RUNNING EXE
  @echo on
  target\debug\trustines.exe
  @echo off
) else (
  echo FILEPATH target\debug\trustines.exe NOT FOUND. DID THE BUILD SUCCEED?
  goto end
)

echo.
echo BUILD AND RUN FINISHED SUCCESSFULLY

:end
echo.
pause
