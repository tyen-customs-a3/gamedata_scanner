@echo off
echo Starting Arma 3 gamedata parsing...

:: Change to the parser code directory (where the Cargo.toml is located)
cd /d "%~dp0"

:: Create output directory structure
mkdir "..\parser_output" 2>nul
mkdir "..\parser_output\failing_files" 2>nul

:: Run the batch parser with the game data folder
cargo run --release --bin batch_parser -- ^
  --input-dir "D:\pca\git\dep\rs\arma3_tool\cache\game_data" ^
  --output-dir "..\parser_output\failing_files" ^
  --report-path "..\parser_output\parse_report.json" ^
  --diagnostic-path "..\parser_output\diagnostic_report.log" ^
  --verbose

echo Parsing complete. Check parser_output folder for results.
pause
