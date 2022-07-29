go clean -cache
cd ./extends/fts/ftslib/
cargo build --release
cbindgen --config cbindgen.toml --crate fts --output fts.h --lang c
cd ../../../