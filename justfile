pg_url := "postgres://season:season@localhost:5432/marketing_monitor"
entity_dir := "./crates/service/src/_entities"
migration_dir := "./crates/migration"

install:
    @if ! command -v cargo-watch &> /dev/null; then \
        echo "Installing cargo-watch..."; \
        cargo binstall cargo-watch; \
    else \
        echo "cargo-watch already installed."; \
    fi
    @if ! command -v sea-orm-cli &> /dev/null; then \
        echo "Installing sea-orm-cli..."; \
        cargo install sea-orm-cli; \
    else \
        echo "sea-orm-cli already installed."; \
    fi

web-install:
    pnpm install

setup:
    just install
    just web-install

server:
    cargo watch -w apps/backend -w crates -x 'run --package backend'
build:
    cargo build --release --package backend

pre-server:
    just build
    ./target/release/backend

web-dev:
    pnpm -F frontend dev
web-build:
    pnpm -F frontend build
web-start:
    pnpm -F frontend start
lint:
    pnpm -F "@mms/*" lint
lint-fix:
    pnpm -F "@mms/*" lint:fix

clean-entity:
    rm -rf {{entity_dir}}

gen-migration table:
    sea-orm-cli migrate generate {{table}} -d {{migration_dir}}/src

gen-entity:
    just clean-entity
    sea-orm-cli generate entity -u {{pg_url}} -o {{entity_dir}}

migrate command:
    sea-orm-cli migrate {{command}} -u {{pg_url}} -d {{migration_dir}}
    just gen-entity
