package store

import (
	"context"
	"database/sql"
	"time"
)

type ProjectAllowedUser struct {
	ProjectID int64 `json:"project_id"`
	UserID    int64 `json:"user_id"`
}

type ProjectAllowedUsersStore struct {
	db *sql.DB
}

func (m *ProjectAllowedUsersStore) AddAllowedUser(projectID, userID int64) error {
	query := `
        INSERT INTO project_allowed_users (project_id, user_id)
        VALUES ($1, $2)
        ON CONFLICT (project_id, user_id) DO NOTHING`

	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()

	_, err := m.db.ExecContext(ctx, query, projectID, userID)
	return err
}

func (m *ProjectAllowedUsersStore) IsUserAllowed(projectID, userID int64) (bool, error) {
	query := `
        SELECT EXISTS (
            SELECT 1 FROM project_allowed_users WHERE project_id = $1 AND user_id = $2
        )`

	var exists bool
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()

	err := m.db.QueryRowContext(ctx, query, projectID, userID).Scan(&exists)
	if err != nil {
		return false, err
	}
	return exists, nil
}

func (m *ProjectAllowedUsersStore) RemoveAllowedUser(projectID, userID int64) error {
	query := `
        DELETE FROM project_allowed_users
        WHERE project_id = $1 AND user_id = $2`

	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()

	_, err := m.db.ExecContext(ctx, query, projectID, userID)
	return err
}
