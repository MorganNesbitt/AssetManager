strip_library:
	cargo run --release strip --input src/resources/Library --output resource_output/stripped/library

strip_desert:
	cargo run --release strip --input src/resources/Desert/ --output resource_output/stripped/desert

pack_desert:
	cargo run --release pack --input resource_output/stripped/desert/ --output resource_output/desert/

pack_library:
	cargo run --release pack --input resource_output/stripped/library/ --output resource_output/library/

pack_all: pack_desert pack_library
strip_all: strip_desert strip_library

recreate_ouput_directories:
	rm -rf resource_output
	mkdir resource_output
	mkdir resource_output/desert
	mkdir resource_output/library
	mkdir resource_output/stripped
	mkdir resource_output/stripped/library
	mkdir resource_output/stripped/desert

from_scratch: recreate_ouput_directories strip_all pack_all
