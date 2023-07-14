run-livenet:
	cargo run --bin deploy --features casper-livenet --no-default-features

test-odra-casper:
	cargo odra test -b casper
