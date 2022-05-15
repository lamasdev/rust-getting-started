docker-compose -f docker/docker-compose.yml -p rust run --rm rust --name rust --rm cargo run --manifest-path "$@"/Cargo.toml
