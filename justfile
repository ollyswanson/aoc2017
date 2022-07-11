set dotenv-load

_default:
	just --list

add day: (fetch day)
	./add_day.sh {{day}}

run day:
	cargo run -r --bin {{day}}

fetch day:
	curl 'https://adventofcode.com/2017/day/{{day}}/input' \
		-H "cookie: session=$SESSION_TOKEN" -o "inputs/day`printf "%02d" {{day}}`.txt"
