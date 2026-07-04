_default:
    cargo just --list -u

init:
    cargo tool --install

lint: lint-check

lint-check:
    cargo clippy --no-deps --all-targets --all-features -- -D warnings

lint-fix:
    cargo clippy --no-deps --all-targets --all-features --fix

format: format-fix

format-check:
    cargo fmt --all -- --check

format-fix:
    cargo fmt --all

fix:
    cargo just format-fix
    cargo just lint-fix

check:
    cargo just format
    cargo just lint
    # Catches the default-feature-set regression that broke `cargo publish` verification for
    # 0.1.0: `just lint`/`just test-all` only ever exercise `--all-features`, which never builds
    # what `cargo add gem` (or `cargo publish`'s own verification build) actually compiles.
    cargo build

    cargo just doc-check

doc:
    cargo doc --all-features --no-deps --open --lib

doc-check:
    RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps

doc-gen:
    cargo clean --doc
    RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
    echo '<meta http-equiv="refresh" content="0;url=gem/index.html">' > target/doc/index.html
    rm target/doc/.lock

test *ARGS:
    cargo tool cargo-nextest run {{ARGS}}

test-doc *ARGS:
    cargo test {{ARGS}} --doc


test-all:
    cargo just test --all-features
    cargo just test-doc --all-features

semver-checks:
    # No --baseline-version pin: see grixy's Justfile for why hardcoding a prerelease baseline
    # is fragile on a pre-1.0 track. Letting cargo-semver-checks auto-select the baseline compares
    # against the latest actual published release instead.
    cargo tool cargo-semver-checks

msrv:
    # `space` requires `std` or `libm` (no default features as of alpha.7), so
    # pin a representative feature set here rather than checking bare defaults.
    cargo tool cargo-hack check --rust-version --workspace --all-targets --ignore-private --features std,bytemuck,serde,blend
    
coverage *ARGS:
    cargo tool cargo-llvm-cov --lib --all-features --open

coverage-gen:
    cargo tool cargo-llvm-cov --lib --all-features --lcov --output-path lcov.info
