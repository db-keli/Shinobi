package internal

import (
	"log"

	"github.com/yeqown/go-qrcode/v2"
	"github.com/yeqown/go-qrcode/writer/standard"
)

type Payload struct {
	Url           string   `json:"url"`
	BuildCommands []string `json:"buildCommands"`
	Token         string   `json:"token"`
}

func Generator(payloadData []byte) {
	qrc, err := qrcode.New(string(payloadData))
	if err != nil {
		log.Fatalf("Failed to create qrcode: %v", err)
	}

	w, err := standard.New("qrcode.png")
	if err != nil {
		log.Fatalf("Failed to create writer: %v", err)
	}

	if err = qrc.Save(w); err != nil {
		log.Fatalf("Failed to save qrcode: %v", err)
	}

	log.Printf("QrCode generated successfully")

}
