FROM golang:latest

WORKDIR /usr/src/gollow

ENV MYSQL=root:Chenhan0804@tcp(127.0.0.1:3306)/gollow?charset=utf8&parseTime=True&loc=Local \
REDIS_ADDR=127.0.0.1:6379 \
REDIS_DB=0 \
TOKEN_KEY=gollow \
GO111MODULE=on \
GOPROXY=https://goproxy.io

COPY . .

RUN go mod tidy \
go build -o /usr/local/bin/gollow

EXPOSE 12900

