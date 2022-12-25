run:
	mkdir -p ./out/
	cp ./static/* ./out/
	cargo run

prod:
	mkdir -p ./out/
	cp ./static/* ./out/
	cargo run --release