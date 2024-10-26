package store

import (
	"encoding/json"
	"fmt"

	"github.com/skip2/go-qrcode"
)

func (p *Project) QRCGenerate() ([]byte, error) {
	payloadData, err := json.Marshal(p)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal project to JSON: %w", err)
	}

	qrCode, err := qrcode.Encode(string(payloadData), qrcode.Medium, 256) // Adjust size as needed
	if err != nil {
		return nil, fmt.Errorf("failed to generate QR code: %w", err)
	}

	return qrCode, nil
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
