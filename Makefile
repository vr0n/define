define:
	cargo build --release && cp ./target/release/define ./define

clean:
	rm -rf ./define

update:
	cargo update

install:
	mkdir -p ~/.config/define
	cp ./gcide.dict ~/.config/define/gcide.dict
	sudo cp -i ./define /usr/local/bin/define
