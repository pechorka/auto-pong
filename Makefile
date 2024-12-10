game: game.rs
	rustc --target wasm32-unknown-unknown -o game.wasm game.rs

serve:
	python3 -m http.server 9999