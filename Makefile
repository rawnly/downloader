release:
	cargo build --release
	tar -czf downloader.tar.gz --directory=./target/release downloader
	SHASUM=$$(shasum -a 256 downloader.tar.gz | cut -d ' ' -f 1) ; \
	sd '\{\{shasum\}\}' "$$SHASUM" downloader.rb

install:
	. .env
	cargo install --path .

