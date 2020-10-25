dev:
	 RUST_BACKTRACE=1 RUST_LOG=debug,actix_web=debug,actix_server=debug cargo run
port:
	 lsof -i:8080