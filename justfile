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

server:
    cargo watch -x 'run --package backend'
build:
    cargo build --release --package backend

pre-server:
    just build
    ./target/release/backend

clean-entity:
    rm -rf {{entity_dir}}

gen-migration table:
    sea-orm-cli migrate generate {{table}} -d {{migration_dir}}/src

gen-entity:
    just clean-entity
    sea-orm-cli generate entity -u {{pg_url}} -s mm_auth -o {{entity_dir}}

migrate command:
    sea-orm-cli migrate {{command}} -u {{pg_url}} -s mm_auth -d {{migration_dir}}
    just gen-entity
