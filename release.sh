# Install cli tool to use 
cargo install diesel_cli --no-default-features --features "postgres" --vers "1.2.0"

diesel migration run
