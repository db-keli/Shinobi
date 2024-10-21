# Use the official Golang image as the base image
FROM golang:1.20 as builder

WORKDIR /app

# Copy the Go module files and download dependencies
COPY go.mod go.sum ./
RUN go mod download

# Copy the rest of the application source code
COPY . .

# Build the application
RUN go build -o server ./cmd/api

# Create a smaller image for the final production stage
FROM alpine:3.17

# Install the golang-migrate tool
RUN apk add --no-cache curl \
    && curl -L https://github.com/golang-migrate/migrate/releases/download/v4.15.2/migrate.linux-amd64.tar.gz | tar xvz \
    && mv migrate /usr/local/bin/migrate

WORKDIR /app

# Copy the server binary from the builder stage
COPY --from=builder /app/server /app/server

# Copy any migration files
COPY migrations /app/migrations

# Run the migrations when the container starts
CMD ["/bin/sh", "-c", "/usr/local/bin/migrate -path /app/migrations -database ${DATABASE_URL} up && /app/server"]
