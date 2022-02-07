all: stlcat

stlcat: src/*.rs
	cargo build --release && cp target/release/stlcat .

clean:
	rm -f stlcat
