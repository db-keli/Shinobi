package store

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"time"
)

type ProjectInput struct {
	Name          string            `json:"name"`
	ProjectUrl    string            `json:"project_url"`
	BuildCommands []string          `json:"build_commands"`
	Keys          map[string]string `json:"keys"`
	ExpireAt      time.Time         `json:"expire_at"`
}

func (pi *ProjectInput) EncryptKeys(key []byte) (string, error) {
	block, err := aes.NewCipher(key)
	if err != nil {
		return "", errors.New("unable to create AES cipher")
	}

	iv := make([]byte, aes.BlockSize)
	if _, err := io.ReadFull(rand.Reader, iv); err != nil {
		return "", errors.New("unable to generate IV")
	}

	keysData, err := json.Marshal(pi.Keys)
	if err != nil {
		return "", errors.New("unable to marshal keys data")
	}

	cipherText := make([]byte, len(keysData))
	stream := cipher.NewCFBEncrypter(block, iv)
	stream.XORKeyStream(cipherText, keysData)

	return base64.StdEncoding.EncodeToString(append(iv, cipherText...)), nil
}

func (pi *ProjectInput) DecryptKeys(encrypted string, key []byte) error {
	ciphertext, err := base64.StdEncoding.DecodeString(encrypted)
	if err != nil {
		return err
	}

	if len(ciphertext) < aes.BlockSize {
		return fmt.Errorf("ciphertext too short")
	}

	iv := ciphertext[:aes.BlockSize]
	ciphertext = ciphertext[aes.BlockSize:]

	block, err := aes.NewCipher(key)
	if err != nil {
		return err
	}

	decrypted := make([]byte, len(ciphertext))

	stream := cipher.NewCFBDecrypter(block, iv)
	stream.XORKeyStream(decrypted, ciphertext)

	return json.Unmarshal(decrypted, &pi.Keys)
}
