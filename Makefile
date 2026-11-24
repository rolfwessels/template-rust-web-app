.DEFAULT_GOAL := help

PROJECT := template-rust-web-app
IMAGE ?= $(PROJECT)
PNPM ?= $(if $(shell command -v pnpm 2>/dev/null),pnpm,corepack pnpm)
VERSION_PREFIX := 0.1
BRANCH := $(or $(GITHUB_HEAD_REF),$(GITHUB_REF_NAME),$(shell git branch --show-current 2>/dev/null),main)
COMMIT_COUNT := $(or $(shell git rev-list --count HEAD 2>/dev/null),0)
SHORT_HASH := $(or $(shell git rev-parse --short=8 HEAD 2>/dev/null),00000000)

ifeq ($(BRANCH),main)
VERSION := $(VERSION_PREFIX).$(COMMIT_COUNT)
IMAGE_TAGS := latest alpha v$(VERSION) $(SHORT_HASH)
else
BASE := $(or $(shell git rev-parse --verify --quiet origin/main),$(shell git rev-parse --verify --quiet main),HEAD)
BASE_COUNT := $(or $(shell git rev-list --count $(BASE) 2>/dev/null),0)
BRANCH_COUNT := $(or $(shell git rev-list --count $(BASE)..HEAD 2>/dev/null),0)
VERSION := $(VERSION_PREFIX).$(BASE_COUNT).$(BRANCH_COUNT)-alpha
IMAGE_TAGS := alpha v$(VERSION) $(SHORT_HASH)
endif

.PHONY: help version build up down install start test test-rust test-web \
	web-build build-release test-browser verify docker-build docker-run

help:
	@printf '%s\n' \
		'make version        Print the Git-derived version' \
		'make build up       Build the dev container and open a shell' \
		'make down           Stop the dev container' \
		'make install        Install locked frontend dependencies' \
		'make start          Build the frontend and start the app' \
		'make test-rust      Run the fast Rust tests' \
		'make test           Run Rust and TypeScript checks and Rust tests' \
		'make build-release  Build the release binary with the UI embedded' \
		'make verify         Run checks and the two-browser test' \
		'make docker-build   Build and tag the runtime image' \
		'make docker-run     Build and run the runtime image with Compose'

version:
	@printf '%s\n' "$(VERSION)"

build:
	docker compose build

up:
	docker compose up -d
	docker compose exec dev bash

down:
	docker compose down

install:
	cd src/web && $(PNPM) install --frozen-lockfile

web-build: install
	cd src/web && $(PNPM) build

start: web-build
	APP_VERSION="$(VERSION)" cargo run --locked --features embed-web -p $(PROJECT)

test-rust:
	cargo test --workspace --locked

test-web: install
	cd src/web && $(PNPM) typecheck && $(PNPM) lint

test: test-rust test-web
	cargo fmt --all --check
	cargo clippy --workspace --all-targets --locked -- -D warnings

build-release: web-build
	APP_VERSION="$(VERSION)" cargo build --release --locked --features embed-web -p $(PROJECT)

test-browser: build-release
	cd src/web && $(PNPM) test:browser

verify: test test-browser

docker-build:
	docker build --target runtime --build-arg VERSION="$(VERSION)" $(foreach tag,$(IMAGE_TAGS),-t $(IMAGE):$(tag)) .

docker-run:
	VERSION="$(VERSION)" docker compose -f src/docker-compose.yml up --build
