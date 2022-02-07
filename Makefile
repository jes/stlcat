all: stlcat

stlcat: src/*.rs
	cargo build --release && cp target/release/stlcat .

install: stlcat
	cp stlcat /usr/bin/stlcat

clean:
	rm -f stlcat
