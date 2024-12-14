game: game.rs
	rustc --target wasm32-unknown-unknown -C link-arg=--export-table -o game.wasm game.rs

serve:
	python3 -m http.server 9999