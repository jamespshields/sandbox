.PHONY: build run test clean docker build-base

build: build-base
	mkdir -p bin
	cargo build --release
	cp target/release/sb bin/
	echo "To add ./bin to PATH, run:"; \
	echo "  export PATH=\"$$(pwd)/bin:\$$PATH\""; \

clean:
	cargo clean
	rm -f bin/sb
	# Stop and remove all sandbox containers (both old and new naming)
	docker ps -a --filter "name=sandbox" --format "{{.Names}}" | xargs -r docker stop 2>/dev/null || true
	docker ps -a --filter "name=sandbox" --format "{{.Names}}" | xargs -r docker rm 2>/dev/null || true
	# Remove sandbox images
	docker rmi sandbox-base 2>/dev/null || true
	docker rmi sandbox-base:latest 2>/dev/null || true
	# Remove sandbox volumes
	docker volume rm sandbox-home 2>/dev/null || true
	# Clean up compose files and .sandbox directory
	rm -rf .sandbox

build-base:
	docker build -f templates/Dockerfile.base -t sandbox-base:latest .
