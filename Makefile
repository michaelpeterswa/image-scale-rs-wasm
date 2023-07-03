all: build

.PHONY: githooks
githooks:
	git config --local core.hooksPath .githooks/

.PHONY: build
build:
	@echo "Building wasm..."
	@wasm-pack build --release --target web --out-dir frontend/pkg
	@echo "Serving on port 8080..."
	@cd frontend && python3 -m http.server 8080