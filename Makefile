test:
	find . -type f -name "*.toml" -exec cargo test  --quiet --manifest-path {} \;

format:	
	find . -type f -name "*.toml" -exec cargo fmt  --quiet --manifest-path {} \;

lint:
	find . -type f -name "*.toml" -exec cargo clippy  --quiet --manifest-path {} \;
