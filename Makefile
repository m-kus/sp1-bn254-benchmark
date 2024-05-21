default:
	cd program && cargo prove build
	cd script && RUST_LOG=debug cargo run --release > output.log
	cat script/output.log | grep "runtime.trace"