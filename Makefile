strip_library:
	cargo run strip --input src/resources/Library --output resource_output/stripped/library/

strip_desert:
	cargo run strip --input src/resources/Desert/ --output resource_output/stripped/desert/

pack_desert:
	cargo run pack --input src/resources/Desert --output resource_output/desert/

pack_library:
	cargo run pack --input src/resources/Library --output resource_output/library/
