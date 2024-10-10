package store

import (
	"encoding/json"
	"fmt"
	"log"

	"github.com/yeqown/go-qrcode/v2"
	"github.com/yeqown/go-qrcode/writer/standard"
)

func (p *Project) QRCGenerate() error {
	// Marshal the Project struct to JSON
	payloadData, err := json.Marshal(p)
	if err != nil {
		return fmt.Errorf("failed to marshal project to JSON: %w", err)
	}

	qrc, err := qrcode.New(string(payloadData))
	if err != nil {
		return fmt.Errorf("failed to create QR code: %w", err)
	}

	w, err := standard.New("qrcode.jpg")
	if err != nil {
		return fmt.Errorf("failed to create writer: %w", err)
	}

	if err = qrc.Save(w); err != nil {
		return fmt.Errorf("failed to save QR code: %w", err)
	}

	log.Printf("QR Code generated successfully")
	return nil
}

// func QRCGenerator(payloadData []byte) {
// 	qrc, err := qrcode.New(string(payloadData))
// 	if err != nil {
// 		log.Fatalf("Failed to create qrcode: %v", err)
// 	}

// 	w, err := standard.New("qrcode.png")
// 	if err != nil {
// 		log.Fatalf("Failed to create writer: %v", err)
// 	}

// 	if err = qrc.Save(w); err != nil {
// 		log.Fatalf("Failed to save qrcode: %v", err)
// 	}

// 	log.Printf("QrCode generated successfully")

// }
