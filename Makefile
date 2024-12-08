game: game.zig
	zig build-exe game.zig \
    	-target wasm32-freestanding \
    	# -O ReleaseSmall \
    	# -fno-entry \
    	# -static \
    	# --no-entry
