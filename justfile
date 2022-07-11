set dotenv-load

_default:
	just --list

add DAY: (fetch DAY)
	./add_day.sh {{DAY}}

run DAY:
	cargo run -r --bin day`printf "%02d" {{DAY}}`

fetch DAY:
	curl 'https://adventofcode.com/2017/day/{{DAY}}/input' \
		-H "cookie: session=$SESSION_TOKEN" -o "inputs/day`printf "%02d" {{DAY}}`.txt"

test DAY:
	cargo test day`printf "%02d" {{DAY}}`
