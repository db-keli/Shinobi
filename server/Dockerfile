FROM golang:1.19 AS builder

WORKDIR /app

COPY . .

RUN go build -o out ./cmd/api

FROM ubuntu:latest

WORKDIR /app

COPY --from=builder /app/out .

CMD ["./out"]
