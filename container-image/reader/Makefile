all:
	cargo build --target=x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/debug/reader .

ctr-build: all
	podman build . -t reader

ctr-export: ctr-build
	podman save --format oci-dir -o ./reader-oci $(ID)

clean:
	cargo clean
	rm -f ./reader
	rm -rf ./reader-oci
	rm -rf *~
