FROM golang:1.20 AS builder

WORKDIR /app

COPY . .

RUN go build -o out ./cmd/api

FROM ubuntu:latest

WORKDIR /app

COPY --from=builder /app/out .

CMD ["./out"]
