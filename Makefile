dev:
	 RUST_BACKTRACE=1 RUST_LOG=debug cargo run
port:
	 lsof -i:8080