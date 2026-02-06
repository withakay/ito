.DEFAULT_GOAL := help

MAX_RUST_FILE_LINES ?= 1000
BUMP ?= none
RUST_WARNINGS_AS_ERRORS ?= -D warnings

.PHONY: \
	init \
	build test test-watch test-coverage lint check check-max-lines clean help \
	arch-guardrails \
	release release-plz-update release-plz-release-pr \
	version-bump version-bump-patch version-bump-minor version-bump-major \
	version-sync \
	rust-build rust-build-release rust-test rust-test-coverage rust-lint rust-install install \
	dev docs docs-open

init: ## Initialize development environment (check rust, install prek hooks)
	@set -e; \
	echo "Checking development dependencies..."; \
	echo ""; \
	MISSING=""; \
	if rustc --version >/dev/null 2>&1; then \
		echo "✓ Rust: $$(rustc --version)"; \
	else \
		echo "✗ Rust: not installed"; \
		echo "  Install: https://rustup.rs/"; \
		MISSING="$$MISSING rust"; \
	fi; \
	if cargo --version >/dev/null 2>&1; then \
		echo "✓ Cargo: $$(cargo --version)"; \
	else \
		echo "✗ Cargo: not installed (comes with Rust)"; \
		MISSING="$$MISSING cargo"; \
	fi; \
	if prek --version >/dev/null 2>&1; then \
		echo "✓ prek: $$(prek --version)"; \
	else \
		echo "✗ prek: not installed"; \
		echo "  Install: cargo install prek"; \
		MISSING="$$MISSING prek"; \
	fi; \
	if python3 --version >/dev/null 2>&1; then \
		echo "✓ Python: $$(python3 --version)"; \
	else \
		echo "✗ Python 3: not installed"; \
		MISSING="$$MISSING python3"; \
	fi; \
	echo ""; \
	if [ -n "$$MISSING" ]; then \
		echo "Missing dependencies:$$MISSING"; \
		echo "Please install them and re-run 'make init'"; \
		exit 1; \
	fi; \
	echo "Installing git hooks via prek..."; \
	prek install; \
	prek install -t pre-push; \
	echo ""; \
	echo "✓ Development environment ready!"; \
	echo ""; \
	echo "Optional tools (install for full functionality):"; \
	if cargo llvm-cov --version >/dev/null 2>&1; then \
		echo "  ✓ cargo-llvm-cov (test coverage)"; \
	else \
		echo "  ○ cargo-llvm-cov: cargo install cargo-llvm-cov"; \
	fi; \
	if cargo watch --version >/dev/null 2>&1; then \
		echo "  ✓ cargo-watch (test watch mode)"; \
	else \
		echo "  ○ cargo-watch: cargo install cargo-watch"; \
	fi; \
	if release-plz --version >/dev/null 2>&1; then \
		echo "  ✓ release-plz (release management)"; \
	else \
		echo "  ○ release-plz: cargo install release-plz"; \
	fi; \
	if gh --version >/dev/null 2>&1; then \
		echo "  ✓ gh CLI (GitHub integration)"; \
	else \
		echo "  ○ gh CLI: https://cli.github.com/"; \
	fi

build: ## Build the project
	$(MAKE) rust-build

test: ## Run tests
	$(MAKE) rust-test

test-watch: ## Run tests in watch mode (requires cargo-watch)
	@set -e; \
	if cargo watch -V >/dev/null 2>&1; then \
		RUSTFLAGS="$(RUST_WARNINGS_AS_ERRORS) $(RUSTFLAGS)" cargo watch -x "test --manifest-path ito-rs/Cargo.toml --workspace"; \
	else \
		echo "cargo-watch is not installed."; \
		echo "Install: cargo install cargo-watch"; \
		exit 1; \
	fi

test-coverage: ## Run coverage (requires cargo-llvm-cov)
	@set -e; \
	if cargo llvm-cov --version >/dev/null 2>&1; then \
		RUSTFLAGS="$(RUST_WARNINGS_AS_ERRORS) $(RUSTFLAGS)" cargo llvm-cov --manifest-path ito-rs/Cargo.toml --workspace --tests; \
	else \
		echo "cargo-llvm-cov is not installed."; \
		echo "Install: cargo install cargo-llvm-cov"; \
		exit 1; \
	fi

lint: ## Run linter
	$(MAKE) rust-lint

check: ## Run pre-commit hooks via prek
	@set -e; \
	if prek --version >/dev/null 2>&1; then \
		prek run --all-files; \
	else \
		echo "prek is not installed."; \
		echo "Install: cargo install prek"; \
		exit 1; \
	fi

check-max-lines: ## Fail if Rust files exceed 1000 lines (override MAX_RUST_FILE_LINES=...)
	python3 "ito-rs/tools/check_max_lines.py" --max-lines "$(MAX_RUST_FILE_LINES)" --root "ito-rs"

arch-guardrails: ## Run architecture guardrail checks
	python3 "ito-rs/tools/arch_guardrails.py"

release: ## Trigger Release Please workflow (creates/updates release PR)
	@set -e; \
	if gh --version >/dev/null 2>&1; then \
		:; \
	else \
		echo "gh is not installed."; \
		echo "Install: https://cli.github.com/"; \
		exit 1; \
	fi; \
	if gh auth status >/dev/null 2>&1; then \
		:; \
	else \
		echo "gh is not authenticated."; \
		echo "Run: gh auth login"; \
		exit 1; \
	fi; \
	CONCLUSION=$$(gh run list --workflow ci.yml --branch main --limit 1 --json conclusion -q '.[0].conclusion' 2>/dev/null || true); \
	if [ "$$CONCLUSION" != "success" ]; then \
		echo "Latest CI run on main is not successful (conclusion=$$CONCLUSION)."; \
		echo "Wait for CI to finish, or rerun CI, then retry."; \
		exit 1; \
	fi; \
	WORKFLOW=release-please.yml; \
	gh workflow run "$$WORKFLOW" --ref main; \
	echo "Triggered Release Please."; \
	echo "Waiting for Release Please PR..."; \
	SLEEP_SECS=2; \
	MAX_TRIES=30; \
	TRY=0; \
	while [ "$$TRY" -lt "$$MAX_TRIES" ]; do \
		PR_URL=$$(gh pr list --state open --head release-please--branches--main --json url -q '.[0].url' 2>/dev/null || true); \
		if [ -z "$$PR_URL" ]; then \
			PR_URL=$$(gh pr list --state open --label "autorelease: pending" --json url -q '.[0].url' 2>/dev/null || true); \
		fi; \
		if [ -n "$$PR_URL" ]; then \
			echo "Release Please PR: $$PR_URL"; \
			exit 0; \
		fi; \
		TRY=$$((TRY + 1)); \
		sleep "$$SLEEP_SECS"; \
	done; \
	echo "Could not find Release Please PR yet."; \
	echo "View runs: gh run list --workflow $$WORKFLOW --branch main --limit 5"; \
	exit 1

version-bump: ## Bump workspace version (BUMP=none|patch|minor|major)
	@set -e; \
	MANIFEST="ito-rs/Cargo.toml"; \
	STAMP=$$(date +%Y%m%d%H%M); \
	NEW_VERSION=$$(python3 "ito-rs/tools/version_bump.py" --manifest "$$MANIFEST" --stamp "$$STAMP" --bump "$(BUMP)"); \
	echo "Bumped workspace version to $$NEW_VERSION"

version-sync: ## Sync workspace/crate versions to Release Please + stamp
	@set -e; \
	STAMP=$$(date +%Y%m%d%H%M); \
	NEW_VERSION=$$(python3 "ito-rs/tools/sync_versions.py" --stamp "$$STAMP"); \
	echo "Synced workspace/crate versions to $$NEW_VERSION"

version-bump-patch: ## Bump patch version (x.y.z -> x.y.(z+1)) + stamp
	$(MAKE) version-bump BUMP=patch

version-bump-minor: ## Bump minor version (x.y.z -> x.(y+1).0) + stamp
	$(MAKE) version-bump BUMP=minor

version-bump-major: ## Bump major version (x.y.z -> (x+1).0.0) + stamp
	$(MAKE) version-bump BUMP=major

rust-build: ## Build Rust ito (debug)
	cargo build --manifest-path ito-rs/Cargo.toml -p ito-cli --bin ito

rust-build-release: ## Build Rust ito (release)
	cargo build --manifest-path ito-rs/Cargo.toml -p ito-cli --bin ito --release

rust-test: ## Run Rust tests
	RUSTFLAGS="$(RUST_WARNINGS_AS_ERRORS) $(RUSTFLAGS)" cargo test --manifest-path ito-rs/Cargo.toml --workspace

rust-test-coverage: ## Run Rust tests with coverage (fallback to regular tests)
	@set -e; \
	if cargo llvm-cov --version >/dev/null 2>&1; then \
		RUSTFLAGS="$(RUST_WARNINGS_AS_ERRORS) $(RUSTFLAGS)" cargo llvm-cov --manifest-path ito-rs/Cargo.toml --workspace --tests; \
	else \
		echo "cargo-llvm-cov is not installed, falling back to regular tests."; \
		echo "Install: cargo install cargo-llvm-cov"; \
		RUSTFLAGS="$(RUST_WARNINGS_AS_ERRORS) $(RUSTFLAGS)" cargo test --manifest-path ito-rs/Cargo.toml --workspace; \
	fi

rust-lint: ## Run Rust fmt/clippy
	cargo fmt --manifest-path ito-rs/Cargo.toml --all -- --check
	cargo clippy --manifest-path ito-rs/Cargo.toml --workspace --all-targets -- \
		-D warnings \
		-D clippy::dbg_macro \
		-D clippy::todo \
		-D clippy::unimplemented

rust-install: ## Install Rust ito as 'ito' into ~/.local/bin (override INSTALL_DIR=...)
	@set -e; \
	$(MAKE) rust-build-release; \
	INSTALL_DIR=$${INSTALL_DIR:-$${HOME}/.local/bin}; \
	mkdir -p "$$INSTALL_DIR"; \
	cp "ito-rs/target/release/ito" "$$INSTALL_DIR/ito"; \
	chmod +x "$$INSTALL_DIR/ito"; \
	if [ "$$(uname -s)" = "Darwin" ]; then \
		codesign --force --sign - "$$INSTALL_DIR/ito"; \
	fi; \
	"$$INSTALL_DIR/ito" --version

install: version-sync rust-install ## Sync version date + install Rust ito as 'ito'

dev: ## Build and install debug version with git info (fast iteration)
	@set -e; \
	cargo build --manifest-path ito-rs/Cargo.toml -p ito-cli --bin ito; \
	INSTALL_DIR=$${INSTALL_DIR:-$${HOME}/.local/bin}; \
	mkdir -p "$$INSTALL_DIR"; \
	cp "ito-rs/target/debug/ito" "$$INSTALL_DIR/ito"; \
	chmod +x "$$INSTALL_DIR/ito"; \
	if [ "$$(uname -s)" = "Darwin" ]; then \
		codesign --force --sign - "$$INSTALL_DIR/ito"; \
	fi; \
	echo "Installed: $$INSTALL_DIR/ito"; \
	"$$INSTALL_DIR/ito" --version

release-plz-update: ## Run release-plz update (bump versions based on commits)
	@set -e; \
	if release-plz --version >/dev/null 2>&1; then \
		release-plz update --manifest-path ito-rs/Cargo.toml --config release-plz.toml; \
	else \
		echo "release-plz is not installed."; \
		echo "Install: cargo install release-plz"; \
		exit 1; \
	fi

release-plz-release-pr: ## Run release-plz release-pr (create/update release PR)
	@set -e; \
	if release-plz --version >/dev/null 2>&1; then \
		release-plz release-pr --manifest-path ito-rs/Cargo.toml --git-token `gh auth token` --config release-plz.toml; \
	else \
		echo "release-plz is not installed."; \
		echo "Install: cargo install release-plz"; \
		exit 1; \
	fi

docs: ## Build Rust documentation (warns on missing docs)
	RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path ito-rs/Cargo.toml --workspace --no-deps

docs-open: ## Build and open Rust documentation in browser
	RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path ito-rs/Cargo.toml --workspace --no-deps --open

clean: ## Remove build artifacts
	rm -rf ito-rs/target

help: ## Show this help message
	@echo "Available targets:" \
	&& awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z0-9_.-]+:.*##/ {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)
