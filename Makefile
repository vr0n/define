define:
	cargo build && cp ./target/debug/define ./define

clean:
	rm -rf ./define

update:
	cargo update
