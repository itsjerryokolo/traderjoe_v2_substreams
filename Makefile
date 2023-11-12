CARGO_VERSION := $(shell cargo version 2>/dev/null)

.PHONY: auth
auth: auth
	./auth.sh

.PHONY: build
build:
ifdef CARGO_VERSION
	cargo build --target wasm32-unknown-unknown --release
else
	@echo "Building substreams target using Docker. To speed up this step, install a Rust development environment."
	docker run --rm -ti --init -v ${PWD}:/usr/src --workdir /usr/src/ rust:bullseye cargo build --target wasm32-unknown-unknown --release
endif

.PHONY: run
run: build
	substreams run substreams.yaml map_events $(if $(START_BLOCK),-s $(START_BLOCK)) $(if $(STOP_BLOCK),-t $(STOP_BLOCK))



.PHONY: gui
gui: build
	substreams gui  substreams.yaml map_factory_events -s 17821282 -t +50000

	# substreams gui substreams.yaml map_factory_events $(if $(START_BLOCK),-s $(START_BLOCK)) $(if $(STOP_BLOCK),-t $(STOP_BLOCK))

.PHONY: codegen
codegen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: pack
pack: build
	substreams pack substreams.yaml
