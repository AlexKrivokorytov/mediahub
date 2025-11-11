@echo off
setlocal EnableExtensions EnableDelayedExpansion

REM Media Hub launcher (Windows CMD)
REM - Ensures Node/npm/cargo presence
REM - Installs deps (incl. Tailwind) if missing
REM - Clears stale Next lock, frees port 3000
REM - Modes: no arg = dev, bundle = build+bundle, clean = remove .next/dist

set PORT=3000
if /I "%~1"=="bundle" goto :bundle
if /I "%~1"=="clean" goto :clean
if /I "%~1"=="doctor" goto :doctor
if /I "%~1"=="repair" goto :repair

call :ensure_node || goto :end
call :ensure_deps || goto :end
call :clear_next_lock
call :free_port

echo [Media Hub] Starting desktop app (Tauri + Next) on port %PORT%...
call npm run app
goto :end

:bundle
call :ensure_node || goto :end
call :ensure_deps || goto :end
echo [Media Hub] Building static export and Tauri bundle...
call npm run bundle
goto :end

:clean
echo [Media Hub] Cleaning build artifacts (.next, dist)...
if exist ".\.next" rmdir /S /Q ".\.next" 2>nul
if exist ".\dist" rmdir /S /Q ".\dist" 2>nul
echo [Media Hub] Clean complete.
goto :end

:doctor
echo [Media Hub] Environment diagnostics
where node  >nul 2>&1 && (for /f "tokens=*" %%v in ('node -v') do echo  - node %%v) || echo  - node: NOT FOUND
where npm   >nul 2>&1 && (for /f "tokens=*" %%v in ('npm -v') do echo  - npm %%v)  || echo  - npm: NOT FOUND
where cargo >nul 2>&1 && (for /f "tokens=*" %%v in ('cargo -V') do echo  - %%v)   || echo  - cargo: NOT FOUND
where rustc >nul 2>&1 && (for /f "tokens=*" %%v in ('rustc -V') do echo  - %%v)   || echo  - rustc: NOT FOUND
if exist ".\node_modules" (echo  - node_modules: PRESENT) else (echo  - node_modules: MISSING)
if exist ".\node_modules\tailwindcss" (echo  - tailwindcss: PRESENT) else (echo  - tailwindcss: MISSING)
if exist ".\.next\dev\lock" (echo  - next lock: PRESENT) else (echo  - next lock: none)
for /f "tokens=5" %%p in ('netstat -ano ^| findstr /R /C:":%PORT% .*LISTENING"') do ( echo  - port %PORT% in use by PID %%p )
for /f "tokens=*" %%v in ('findstr /C:"\"devUrl\"" src-tauri\tauri.conf.json') do echo  - tauri devUrl: %%v
for /f "tokens=*" %%v in ('npx tauri -V 2^>nul') do echo  - tauri cli: %%v
echo  - next script: "npm run dev" forces port %PORT%
echo [Media Hub] Tip: run "run-media-hub clean" then "run-media-hub" if issues persist.
goto :end

:repair
echo [Media Hub] Repairing local setup...
call :clean
echo [Media Hub] Reinstalling dependencies...
if exist package-lock.json del /f /q package-lock.json >nul 2>&1
npm install || exit /b 1
echo [Media Hub] Repair complete. Try running without args.
goto :end

:ensure_node
where node >nul 2>&1 || ( echo [Media Hub] ERROR: Node.js not found in PATH.& exit /b 1 )
where npm  >nul 2>&1 || ( echo [Media Hub] ERROR: npm not found in PATH.& exit /b 1 )
where cargo >nul 2>&1 || ( echo [Media Hub] WARNING: cargo not found. Tauri dev/build may fail. )
where rustc >nul 2>&1 || ( echo [Media Hub] WARNING: rustc not found. Tauri dev/build may fail. )
for /f "tokens=*" %%v in ('node -v') do set NODE_VER=%%v
for /f "tokens=*" %%v in ('npm -v') do set NPM_VER=%%v
echo [Media Hub] Using Node !NODE_VER!, npm !NPM_VER!
exit /b 0

:ensure_deps
if not exist node_modules (
  echo [Media Hub] Installing npm dependencies (npm ci)...
  npm ci || (
    echo [Media Hub] npm ci failed, falling back to npm install...
    npm install || exit /b 1
  )
) else (
  if not exist ".\node_modules\tailwindcss" (
    echo [Media Hub] Installing Tailwind/PostCSS deps...
    npm install || exit /b 1
  )
)
exit /b 0

:clear_next_lock
if exist ".\.next\dev\lock" (
  echo [Media Hub] Removing stale Next.js dev lock...
  del /f /q ".\.next\dev\lock" >nul 2>&1
)
exit /b 0

:free_port
set KILLED=0
for /f "tokens=5" %%p in ('netstat -ano ^| findstr /R /C:":%PORT% .*LISTENING"') do (
  echo [Media Hub] Port %PORT% in use by PID %%p. Terminating...
  taskkill /F /PID %%p >nul 2>&1 && set KILLED=1
)
if "%KILLED%"=="1" (
  timeout /t 1 >nul
)
exit /b 0

:end
endlocal
