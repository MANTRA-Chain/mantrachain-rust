# Prints the list of recipes.
default:
    @just --list

# Builds the whole project.
build:
  cargo build

# Tests the whole project.
test:
  cargo test

# Alias to the format recipe.
fmt:
  @just format

# Formats the rust, toml and sh files in the project.
format:
  cargo fmt --all
  find . -type f -iname "*.toml" -print0 | xargs -0 taplo format

# Runs clippy with the a feature flag if provided.
lint:
  cargo clippy --all -- -D warnings

# Tries to fix clippy issues automatically.
lintfix:
  cargo clippy --fix --allow-staged --allow-dirty --all-features
  just format

# Checks the whole project with all the feature flags.
check-all:
  cargo check --all-features

# Cargo check.
check:
  cargo check

# Cargo clean and update.
refresh:
  cargo clean && cargo update

# Cargo watch.
watch:
  cargo watch -x lcheck

# Installs pre-commit git hooks.
install-git-hooks:
    ./scripts/hooks/commit-msg.sh --install
    ./scripts/hooks/pre-commit.sh --install
