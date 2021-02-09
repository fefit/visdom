#! /bin/bash
# build env
# node
echo "Build node env:"
cd ./node
yarn --silent
# go
echo "Build go env:"
cd ../go
export GOPATH=$GOPATH:${PWD}/src
GOMODULE111=true go get -u github.com/PuerkitoBio/goquery
# rust
echo "Build rust env:"
cd ../rust
cargo build --quiet
# run
# cheerio
printf "\n---------------------Cheerio in Node------------------\n"
cd ../node
node index.js
# goquery
printf "\n---------------------Goquery in Go:------------------\n"
cd ../go
go run src/main.go
# visdom
printf "\n"
printf "\n---------------------Visdom in Rust------------------\n"
cd ../rust
cargo run --release --quiet