@_default:
	just --list

# create a new solution for a given day
@start day:
	cargo init day-{{day}}
	touch day-{{day}}/src/input.txt

[no-exit-message]
@run day:
	cargo run --bin day-{{day}}
