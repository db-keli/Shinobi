package store

import (
	"context"
	"database/sql"

	"github.com/lib/pq"
)

type Project struct {
	ID            int      `json:"id"`
	Name          string   `json:"name"`
	UserID        int64    `json:"user_id"`
	ProjectUrl    string   `json:"project_url"`
	BuildCommands []string `json:"build_commands"`
	CreatedAt     string   `json:"created_at"`
	UpdatedAt     string   `json:"updated_at"`
}

type ProjectStore struct {
	db *sql.DB
}

func (s *ProjectStore) Create(ctx context.Context, project *Project) error {
	query := `
	    INSERT INTO projects (name, project_url, build_commands)
		VALUES ($1, $2, $3) RETURNING id, created_at, updated_at
	`

	err := s.db.QueryRowContext(
		ctx, query,
		project.Name,
		project.ProjectUrl,
		pq.Array(project.BuildCommands),
	).Scan(&project.ID, &project.CreatedAt, &project.UpdatedAt)

	if err != nil {
		return err
	}

	return nil
}
