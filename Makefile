run:
	export 
	cargo build --release
	startx ./target/release/rdwm -- :1 vt2
