package main

import "github.com/db-keli/shinobi/internal"

func main() {
	payload := []byte(`{
        "url": "https://example.com",
        "buildCommands": ["go build", "go test"],
        "token": "secret"
    }`)

	internal.Generator(payload)
}
