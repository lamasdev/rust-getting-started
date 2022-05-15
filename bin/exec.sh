docker-compose -f docker/docker-compose.yml -p rust run --rm --name rust rust cargo run --manifest-path $@/Cargo.toml
