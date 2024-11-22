package store

import (
	"context"
	"database/sql"
	"fmt"
	"strconv"
	"time"

	"github.com/golang-jwt/jwt/v4"
	"github.com/google/uuid"
)

var (
	jwtSecretKey = []byte("secret")
)

type KeysToken struct {
	ID        int64     `json:"id"`
	Jti       string    `json:"jti"`
	UserID    int64     `json:"user_id"`
	ProjectID int64     `json:"project_id"`
	IssueAt   time.Time `json:"issued_at"`
	Expiry    time.Time `json:"expires_at"`
	Revoked   bool      `json:"revoked"`
}

func (m *KeysTokenStore) GenerateJWT(userID, projectID string) (string, error) {
	key := jwtSecretKey
	jti := uuid.NewString()

	claims := jwt.MapClaims{
		"user_id":    userID,
		"project_id": projectID,
		"exp":        time.Now().Add(24 * time.Hour).Unix(),
		"jti":        jti,
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	signedToken, err := token.SignedString(key)
	if err != nil {
		return "", fmt.Errorf("failed to sign token: %w", err)
	}

	userIDInt, err := parseToInt64(userID)
	if err != nil {
		return "", fmt.Errorf("invalid userID: %w", err)
	}
	projectIDInt, err := parseToInt64(projectID)
	if err != nil {
		return "", fmt.Errorf("invalid projectID: %w", err)
	}

	err = m.SaveToken(KeysToken{
		Jti:       jti,
		UserID:    userIDInt,
		ProjectID: projectIDInt,
		IssueAt:   time.Now(),
		Expiry:    time.Now().Add(24 * time.Hour),
	})
	if err != nil {
		return "", fmt.Errorf("failed to save token metadata: %w", err)
	}

	return signedToken, nil
}

func parseToInt64(value string) (int64, error) {
	id, err := strconv.ParseInt(value, 10, 64)
	if err != nil {
		return 0, err
	}
	return id, nil
}

func (m *KeysTokenStore) ValidateJWT(tokenString string) (jwt.MapClaims, error) {
	key := jwtSecretKey

	token, err := jwt.Parse(tokenString, func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
		}
		return key, nil
	})

	if err != nil || !token.Valid {
		return nil, fmt.Errorf("invalid token: %w", err)
	}

	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok {
		return nil, fmt.Errorf("invalid claims format")
	}

	// Check for revocation
	jti, ok := claims["jti"].(string)
	if !ok {
		return nil, fmt.Errorf("missing jti in claims")
	}

	isRevoked, err := m.IsTokenRevoked(jti)
	if err != nil {
		return nil, fmt.Errorf("failed to check token revocation: %w", err)
	}
	if isRevoked {
		return nil, fmt.Errorf("token is revoked")
	}

	return claims, nil
}

type KeysTokenStore struct {
	db *sql.DB
}

func (k *KeysTokenStore) New(userID int64, projectID int64, ttl time.Duration) (*KeysToken, error) {
	jti := uuid.NewString()
	expiry := time.Now().Add(ttl)

	token := KeysToken{
		Jti:       jti,
		UserID:    userID,
		ProjectID: projectID,
		IssueAt:   time.Now(),
		Expiry:    expiry,
	}

	err := k.SaveToken(token)
	return &token, err
}

func (m *KeysTokenStore) SaveToken(token KeysToken) error {
	query := `
        INSERT INTO keys_tokens (jti, user_id, project_id, issued_at, expires_at, revoked)
        VALUES ($1, $2, $3, $4, $5, $6)
    `
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()

	_, err := m.db.ExecContext(ctx, query, token.Jti, token.UserID, token.ProjectID, token.IssueAt, token.Expiry, token.Revoked)
	return err
}

func (m *KeysTokenStore) IsTokenRevoked(jti string) (bool, error) {
	query := `SELECT revoked FROM keys_tokens WHERE jti = $1`
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()

	var revoked bool
	err := m.db.QueryRowContext(ctx, query, jti).Scan(&revoked)
	if err != nil {
		if err == sql.ErrNoRows {
			return false, fmt.Errorf("token not found")
		}
		return false, fmt.Errorf("database error: %w", err)
	}

	return revoked, nil
}

func (m *KeysTokenStore) RevokeToken(jti string) error {
	query := `UPDATE keys_tokens SET revoked = TRUE WHERE jti = $1`
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()

	_, err := m.db.ExecContext(ctx, query, jti)
	return err
}
