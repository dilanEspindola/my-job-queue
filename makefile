dev:
	cargo watch --ignore test.sock -x run

daemon:
	cargo run --bin monitor