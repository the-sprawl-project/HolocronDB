.PHONY: sync-protos sync-protos-local push-protos build dev

# Sync from GitHub (respects proto.lock)
sync-protos:
	@./scripts/sync-protos.sh --remote

# Sync from local sprawl-protocols repo for development
sync-protos-local:
	@if [ -z "$(SPRAWL_PROTOCOLS_LOCAL_PATH)" ]; then \
		echo "Error: SPRAWL_PROTOCOLS_LOCAL_PATH not set"; \
		echo "Usage: SPRAWL_PROTOCOLS_LOCAL_PATH=/path/to/sprawl-protocols make sync-protos-local"; \
		exit 1; \
	fi
	@./scripts/sync-protos.sh --local "$(SPRAWL_PROTOCOLS_LOCAL_PATH)"

# Push current protos back to local sprawl-protocols repo
push-protos:
	@if [ -z "$(SPRAWL_PROTOCOLS_LOCAL_PATH)" ]; then \
		echo "Error: SPRAWL_PROTOCOLS_LOCAL_PATH not set"; \
		echo "Usage: SPRAWL_PROTOCOLS_LOCAL_PATH=/path/to/sprawl-protocols make push-protos"; \
		exit 1; \
	fi
	@./scripts/sync-protos.sh --push "$(SPRAWL_PROTOCOLS_LOCAL_PATH)"

# Build with remote protos
build: sync-protos
	cargo build --release

# Development build with local protos
dev:
	@if [ -n "$(SPRAWL_PROTOCOLS_LOCAL_PATH)" ]; then \
		$(MAKE) sync-protos-local; \
	else \
		$(MAKE) sync-protos; \
	fi
	cargo build

# Watch mode for development
watch:
	@if [ -n "$(SPRAWL_PROTOCOLS_LOCAL_PATH)" ]; then \
		$(MAKE) sync-protos-local; \
	fi
	cargo watch -x build