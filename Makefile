build: 
	cargo build --release --offline

run: build
	./target/release/homomorphic_game_of_life

clippy:
	cargo clippy --offline

clean:
	rm -r target; rm Cargo.lock
