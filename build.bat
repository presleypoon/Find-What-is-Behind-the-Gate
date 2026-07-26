@echo off
cargo run
if %errorlevel% neq 0 (
	goto err
)
copy target\debug\find_what_is_behind_the_gate.exe .
pause
exit
:err
