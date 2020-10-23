dev:
	 RUST_BACKTRACE=1 RUST_LOG=trace,actix_web=debug,actix_server=trace cargo run
port:
	 lsof -i:8080