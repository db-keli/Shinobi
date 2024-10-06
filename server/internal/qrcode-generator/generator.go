package qrcodegenerator

import (
	"log"
	"time"

	"github.com/golang-jwt/jwt/v4"
	"github.com/yeqown/go-qrcode/v2"
	"github.com/yeqown/go-qrcode/writer/standard"
)

type Payload struct {
	Url           string   `json:"url"`
	BuildCommands []string `json:"buildCommands"`
	Token         string   `json:"token"`
}

func QRCGenerator(payloadData []byte) {
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

var secretkey = []byte("secret")

func generateToken(payload map[string]interface{}) (string, error) {
	token := jwt.New(jwt.SigningMethodHS256)

	claims := token.Claims.(jwt.MapClaims)

	for key, value := range payload {
		claims[key] = value
	}

	claims["exp"] = time.Now().Add(time.Hour * 24).Unix()

	tokenString, err := token.SignedString(secretkey)
	if err != nil {
		return "", err
	}

	return tokenString, nil
}
