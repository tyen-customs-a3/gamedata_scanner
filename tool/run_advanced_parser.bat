@echo off
echo Starting Arma 3 gamedata parsing with ADVANCED parser...

:: Change to the parser code directory (where the Cargo.toml is located)
cd /d "%~dp0"

:: Create output directory structure
mkdir ".\parser_output_advanced" 2>nul
mkdir ".\parser_output_advanced\failing_files" 2>nul

:: Use advanced parser
set PARSER_TYPE=advanced

:: Run the batch parser with the game data folder
cargo run --release --bin batch_check -- ^
  --input-dir "D:\pca\git\dep\rs\arma3_tool\cache\gamedata" ^
  --output-dir ".\parser_output_advanced\failing_files" ^
  --report-path ".\parser_output_advanced\report.json" ^
  --diagnostic-path ".\parser_output_advanced\diagnostics.log" ^
  --file-extensions "hpp,cpp,h,c" ^
  --max-files 5000 ^
  --max-failures 50 ^
  --parser-type %PARSER_TYPE% ^
  --parallel ^
  --copy-failed-files

echo Parsing complete. Check parser_output_advanced folder for results.
pause 