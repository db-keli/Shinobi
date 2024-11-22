package main

import (
	"encoding/json"
	"net/http"
	"time"

	"github.com/db-keli/shinobi/internal/store"
)

type GenerateTokenRequest struct {
	UserID    string `json:"user_id"`
	ProjectID string `json:"project_id"`
}

type GenerateTokenResponse struct {
	Token     string    `json:"token"`
	ExpiresAt time.Time `json:"expires_at"`
}

func (app *application) GenerateTokenHandler(store *store.KeysTokenStore) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		var req GenerateTokenRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Invalid request payload", http.StatusBadRequest)
			return
		}

		if req.UserID == "" || req.ProjectID == "" {
			http.Error(w, "user_id and project_id are required", http.StatusBadRequest)
			return
		}

		token, err := store.GenerateJWT(req.UserID, req.ProjectID)
		if err != nil {
			http.Error(w, "Failed to generate token", http.StatusInternalServerError)
			return
		}

		expiry := time.Now().Add(24 * time.Hour) // Match the TTL used in the JWT
		resp := GenerateTokenResponse{
			Token:     token,
			ExpiresAt: expiry,
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(resp)
	}
}
