SET PATH=%PATH%;./extends/fts/ftslib/target/release
protoc --go_out=. ./protobuf/*
go build -o go_run.exe main.go  && go_run.exe