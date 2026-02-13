.DEFAULT_GOAL := help

MAX_RUST_FILE_LINES ?= 1000
BUMP ?= none
RUST_WARNINGS_AS_ERRORS ?= -D warnings
COVERAGE_HARD_MIN ?= 80
COVERAGE_TARGET ?= 90
SCCACHE_BIN ?= $(shell command -v sccache 2>/dev/null)
RUSTC_WRAPPER_ENV := $(if $(SCCACHE_BIN),RUSTC_WRAPPER="$(SCCACHE_BIN)")

.PHONY: \
	init \
	build test test-timed test-watch test-coverage lint check check-max-lines clean help \
	fmt clippy \
	arch-guardrails cargo-deny \
	config-schema config-schema-check \
	release release-plz-update release-plz-release-pr \
	version-bump version-bump-patch version-bump-minor version-bump-major \
	version-sync \
	rust-build rust-build-release rust-test rust-test-timed rust-test-coverage rust-lint rust-install install \
	dev docs docs-open docs-site-install docs-site-build docs-site-serve docs-site-check

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
	if rustup --version >/dev/null 2>&1; then \
		echo "Ensuring Rust components (rustfmt, clippy, rust-docs)..."; \
		if rustup component add rustfmt clippy rust-docs; then \
			echo "  ✓ Rust components ready"; \
		else \
			echo "  ○ Rust component install failed: rustup component add rustfmt clippy rust-docs"; \
		fi; \
	else \
		echo "  ○ rustup not found: cannot auto-install rust-docs/rustfmt/clippy"; \
	fi; \
	if cargo llvm-cov --version >/dev/null 2>&1; then \
		echo "  ✓ cargo-llvm-cov (test coverage)"; \
	else \
		echo "Installing cargo-llvm-cov (test coverage)..."; \
		if cargo install cargo-llvm-cov; then \
			echo "  ✓ cargo-llvm-cov installed"; \
		else \
			echo "  ○ cargo-llvm-cov install failed: cargo install cargo-llvm-cov"; \
		fi; \
	fi; \
	if sccache --version >/dev/null 2>&1; then \
		echo "  ✓ sccache (Rust compiler cache)"; \
	else \
		echo "Installing sccache (Rust compiler cache)..."; \
		if cargo install sccache; then \
			echo "  ✓ sccache installed"; \
		else \
			echo "  ○ sccache install failed: cargo install sccache"; \
		fi; \
	fi; \
	if cargo deny --version >/dev/null 2>&1; then \
		echo "  ✓ cargo-deny (license/advisory checks)"; \
	else \
		echo "Installing cargo-deny (license/advisory checks)..."; \
		if cargo install cargo-deny; then \
			echo "  ✓ cargo-deny installed"; \
		else \
			echo "  ○ cargo-deny install failed: cargo install cargo-deny"; \
		fi; \
	fi; \
	if cargo watch --version >/dev/null 2>&1; then \
		echo "  ✓ cargo-watch (test watch mode)"; \
	else \
		echo "Installing cargo-watch (test watch mode)..."; \
		if cargo install cargo-watch; then \
			echo "  ✓ cargo-watch installed"; \
		else \
			echo "  ○ cargo-watch install failed: cargo install cargo-watch"; \
		fi; \
	fi; \
	if release-plz --version >/dev/null 2>&1; then \
		echo "  ✓ release-plz (release management)"; \
	else \
		echo "Installing release-plz (release management)..."; \
		if cargo install release-plz; then \
			echo "  ✓ release-plz installed"; \
		else \
			echo "  ○ release-plz install failed: cargo install release-plz"; \
		fi; \
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

test-timed: ## Run tests with per-crate timing
	$(MAKE) rust-test-timed

test-affected: ## Run tests only for crates affected by recent changes
	bash ito-rs/tools/test-affected.sh

test-watch: ## Run tests in watch mode (requires cargo-watch)
	@set -e; \
	if cargo watch -V >/dev/null 2>&1; then \
		RUSTFLAGS="$(RUST_WARNINGS_AS_ERRORS) $(RUSTFLAGS)" cargo watch -x "test --workspace"; \
	else \
		echo "cargo-watch is not installed."; \
		echo "Install: cargo install cargo-watch"; \
		exit 1; \
	fi

test-coverage: ## Run coverage — fails below $(COVERAGE_HARD_MIN)% (target $(COVERAGE_TARGET)%)
	@set -e; \
	if cargo llvm-cov --version >/dev/null 2>&1; then \
		echo "Coverage enforcement: hard min=$(COVERAGE_HARD_MIN)%, target=$(COVERAGE_TARGET)%"; \
		echo "  Below $(COVERAGE_HARD_MIN)%: build FAILS (hard floor)"; \
		echo "  Below $(COVERAGE_TARGET)%: WARNING (target)"; \
		echo "  Excluded crates: ito-web (no tests yet)"; \
		echo ""; \
		$(RUSTC_WRAPPER_ENV) CARGO_BUILD_JOBS=$${CARGO_BUILD_JOBS:-4} RUSTFLAGS="$(RUST_WARNINGS_AS_ERRORS) $(RUSTFLAGS)" cargo llvm-cov --workspace --tests \
			--exclude ito-web \
			--fail-under-lines $(COVERAGE_HARD_MIN) \
			--fail-under-regions $(COVERAGE_HARD_MIN); \
	else \
		echo "cargo-llvm-cov is not installed."; \
		echo "Install: cargo install cargo-llvm-cov"; \
		exit 1; \
	fi

lint: ## Run linter
	$(MAKE) rust-lint

fmt: ## Run cargo fmt (auto-fix)
	cargo fmt --all

clippy: ## Run cargo clippy
	cargo clippy --workspace --all-targets -- \
		-D warnings \
		-D clippy::dbg_macro \
		-D clippy::todo \
		-D clippy::unimplemented

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

config-schema: ## Generate canonical Ito config JSON schema artifact
	cargo run -p ito-cli --bin ito -- config schema --output schemas/ito-config.schema.json

config-schema-check: ## Verify canonical Ito config schema artifact is up-to-date
	@set -e; \
	if ! cargo run -p ito-cli --bin ito -- config schema | diff -q schemas/ito-config.schema.json - >/dev/null; then \
		echo "schemas/ito-config.schema.json is stale. Run: make config-schema"; \
		exit 1; \
	fi

cargo-deny: ## Run cargo-deny license/advisory checks (requires cargo-deny)
	@set -e; \
	if cargo deny --version >/dev/null 2>&1; then \
		cargo deny check; \
	else \
		echo "cargo-deny is not installed."; \
		echo "Install: cargo install cargo-deny"; \
		exit 1; \
	fi

release: ## Create/update release PR via release-plz
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
	$(MAKE) release-plz-release-pr

version-bump: ## Bump workspace version (BUMP=none|patch|minor|major)
	@set -e; \
	MANIFEST="Cargo.toml"; \
	STAMP=$$(date +%Y%m%d%H%M); \
	NEW_VERSION=$$(python3 "ito-rs/tools/version_bump.py" --manifest "$$MANIFEST" --stamp "$$STAMP" --bump "$(BUMP)"); \
	echo "Bumped workspace version to $$NEW_VERSION"

version-sync: ## Sync workspace/crate versions to workspace version + stamp
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
	$(RUSTC_WRAPPER_ENV) cargo build -p ito-cli --bin ito

rust-build-release: ## Build Rust ito (release)
	$(RUSTC_WRAPPER_ENV) cargo build -p ito-cli --bin ito --release

rust-test: ## Run Rust tests (full cargo test, includes doctests)
	$(RUSTC_WRAPPER_ENV) RUSTFLAGS="$(RUST_WARNINGS_AS_ERRORS) $(RUSTFLAGS)" cargo test --workspace --exclude ito-web

rust-test-timed: ## Run Rust tests with timing
	@set -e; \
	START=$$(date +%s); \
	echo "Running tests with cargo test..."; \
	$(RUSTC_WRAPPER_ENV) RUSTFLAGS="$(RUST_WARNINGS_AS_ERRORS) $(RUSTFLAGS)" cargo test --workspace --exclude ito-web; \
	END=$$(date +%s); \
	echo ""; \
	echo "Total wall time: $$(( END - START ))s"

rust-test-coverage: ## Run Rust tests with coverage + hard floor (fallback to regular tests)
	@set -e; \
	if cargo llvm-cov --version >/dev/null 2>&1; then \
		$(RUSTC_WRAPPER_ENV) CARGO_BUILD_JOBS=$${CARGO_BUILD_JOBS:-4} RUSTFLAGS="$(RUST_WARNINGS_AS_ERRORS) $(RUSTFLAGS)" cargo llvm-cov --workspace --tests \
			--exclude ito-web \
			--fail-under-lines $(COVERAGE_HARD_MIN) \
			--fail-under-regions $(COVERAGE_HARD_MIN); \
	else \
		echo "cargo-llvm-cov is not installed, falling back to regular tests."; \
		echo "Install: cargo install cargo-llvm-cov"; \
		$(RUSTC_WRAPPER_ENV) RUSTFLAGS="$(RUST_WARNINGS_AS_ERRORS) $(RUSTFLAGS)" cargo test --workspace --exclude ito-web; \
	fi

rust-lint: ## Run Rust fmt/clippy
	cargo fmt --all -- --check
	$(RUSTC_WRAPPER_ENV) cargo clippy --workspace --all-targets -- \
		-D warnings \
		-D clippy::dbg_macro \
		-D clippy::todo \
		-D clippy::unimplemented

rust-install: ## Install Rust ito as 'ito' into ~/.local/bin (override INSTALL_DIR=...)
	@set -e; \
	$(MAKE) rust-build-release; \
	INSTALL_DIR=$${INSTALL_DIR:-$${HOME}/.local/bin}; \
	mkdir -p "$$INSTALL_DIR"; \
	cp "target/release/ito" "$$INSTALL_DIR/ito"; \
	chmod +x "$$INSTALL_DIR/ito"; \
	if [ "$$(uname -s)" = "Darwin" ]; then \
		codesign --force --sign - "$$INSTALL_DIR/ito"; \
	fi; \
	"$$INSTALL_DIR/ito" --version

install: version-sync rust-install ## Sync workspace version stamp + install Rust ito as 'ito'

dev: ## Build and install debug version with git info (fast iteration)
	@set -e; \
	$(RUSTC_WRAPPER_ENV) cargo build -p ito-cli --bin ito; \
	INSTALL_DIR=$${INSTALL_DIR:-$${HOME}/.local/bin}; \
	mkdir -p "$$INSTALL_DIR"; \
	cp "target/debug/ito" "$$INSTALL_DIR/ito"; \
	chmod +x "$$INSTALL_DIR/ito"; \
	if [ "$$(uname -s)" = "Darwin" ]; then \
		codesign --force --sign - "$$INSTALL_DIR/ito"; \
	fi; \
	echo "Installed: $$INSTALL_DIR/ito"; \
	"$$INSTALL_DIR/ito" --version

release-plz-update: ## Run release-plz update (bump versions based on commits)
	@set -e; \
	if release-plz --version >/dev/null 2>&1; then \
		release-plz update --config release-plz.toml; \
	else \
		echo "release-plz is not installed."; \
		echo "Install: cargo install release-plz"; \
		exit 1; \
	fi

release-plz-release-pr: ## Run release-plz release-pr (create/update release PR)
	@set -e; \
	if release-plz --version >/dev/null 2>&1; then \
		release-plz release-pr --git-token `gh auth token` --config release-plz.toml; \
	else \
		echo "release-plz is not installed."; \
		echo "Install: cargo install release-plz"; \
		exit 1; \
	fi

docs: ## Build Rust documentation (warns on missing docs)
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps

docs-open: ## Build and open Rust documentation in browser
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --open

docs-site-install: ## Install MkDocs docs-site dependencies
	python3 -m pip install -r docs/requirements.txt

docs-site-build: docs-site-install ## Build MkDocs docs site (strict)
	python3 -m mkdocs build --strict

docs-site-serve: docs-site-install ## Serve MkDocs docs site locally
	python3 -m mkdocs serve

docs-site-check: docs-site-build ## Validate docs site build
	@echo "Docs site check passed"

clean: ## Remove build artifacts
	rm -rf target ito-rs/target

help: ## Show this help message
	@echo "Available targets:" \
	&& awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z0-9_.-]+:.*##/ {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)
