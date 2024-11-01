package store

import (
	"database/sql"
	"errors"
	"time"

	"github.com/db-keli/shinobi/internal/validator"
	"github.com/lib/pq"
)

type Project struct {
	ID            int64     `json:"id"`
	Name          string    `json:"name"`
	UserID        int64     `json:"user_id"`
	ProjectUrl    string    `json:"project_url"`
	BuildCommands []string  `json:"build_commands"`
	Keystoken     string    `json:"keys_token"`
	ExpireAt      time.Time `json:"expire_at"`
	CreatedAt     time.Time `swaggertype:"string"`
	UpdatedAt     time.Time `swaggertype:"string"`
}

type ProjectStore struct {
	db *sql.DB
}

func ValidateProject(v *validator.Validator, project *Project) {
	v.Check(project.ProjectUrl != "", "project_url", "must be provided")
}

func (p ProjectStore) Insert(project *Project) error {
	query := `
        INSERT INTO projects (name, user_id, project_url, build_commands, keys_token, expire_at, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id`

	args := []any{project.Name, project.UserID, project.ProjectUrl, pq.Array(project.BuildCommands), project.Keystoken, project.ExpireAt, project.CreatedAt, project.UpdatedAt}

	return p.db.QueryRow(query, args...).Scan(&project.ID)
}

func (p ProjectStore) Get(id int64) (*Project, error) {
	if id < 1 {
		return nil, ErrRecordNotFound
	}

	query := `
        SELECT id, name, user_id, project_url, build_commands, keys_token, expire_at, created_at, updated_at
        FROM projects
        WHERE id = $1`

	var project Project

	err := p.db.QueryRow(query, id).Scan(
		&project.ID,
		&project.Name,
		&project.UserID,
		&project.ProjectUrl,
		pq.Array(&project.BuildCommands),
		&project.Keystoken,
		&project.ExpireAt,
		&project.CreatedAt,
		&project.UpdatedAt,
	)
	if err != nil {
		switch {
		case errors.Is(err, sql.ErrNoRows):
			return nil, ErrRecordNotFound
		default:
			return nil, err
		}
	}

	return &project, nil
}

func (p ProjectStore) GetByName(name string) (*Project, error) {
	if name == "" {
		return nil, ErrRecordNotFound
	}

	query := `
        SELECT id, name, user_id, project_url, build_commands, keys_token, expire_at, created_at, updated_at
        FROM projects
        WHERE name = $1`

	var project Project

	err := p.db.QueryRow(query, name).Scan(
		&project.ID,
		&project.Name,
		&project.UserID,
		&project.ProjectUrl,
		pq.Array(&project.BuildCommands),
		&project.Keystoken,
		&project.ExpireAt,
		&project.CreatedAt,
		&project.UpdatedAt,
	)
	if err != nil {
		switch {
		case errors.Is(err, sql.ErrNoRows):
			return nil, ErrRecordNotFound
		default:
			return nil, err
		}
	}
	return &project, nil
}

func (p ProjectStore) Update(project *Project) error {
	return nil
}

func (p ProjectStore) Delete(id int64) error {
	if id < 1 {
		return ErrRecordNotFound
	}

	query := `
        DELETE FROM projects
        WHERE id = $1`

	result, err := p.db.Exec(query, id)
	if err != nil {
		return err
	}

	rowsAffected, err := result.RowsAffected()
	if err != nil {
		return err
	}

	if rowsAffected == 0 {
		return ErrRecordNotFound
	}

	return nil
}

func (p ProjectStore) GetAll(user_id int64) (*Project, error) {
	if user_id < 1 {
		return nil, ErrRecordNotFound
	}

	query := `
        SELECT id, name, user_id, project_url, build_commands, keys_token, expire_at, created_at, updated_at
        FROM projects
        WHERE user_id = $1`

	var project Project

	err := p.db.QueryRow(query, user_id).Scan(
		&project.ID,
		&project.Name,
		&project.UserID,
		&project.ProjectUrl,
		pq.Array(&project.BuildCommands),
		&project.Keystoken,
		&project.ExpireAt,
		&project.CreatedAt,
		&project.UpdatedAt,
	)

	if err != nil {
		switch {
		case errors.Is(err, sql.ErrNoRows):
			return nil, ErrRecordNotFound
		default:
			return nil, err
		}
	}
	return &project, nil
}
