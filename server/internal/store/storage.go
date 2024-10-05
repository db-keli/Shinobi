package store

import (
	"context"
	"database/sql"
)

type Storage struct {
	Projects interface {
		Create(context.Context, *Project) error
	}

	Users interface {
		Create(context.Context, *User) error
	}
}

func NewPostgresStorage(db *sql.DB) Storage {
	return Storage{
		Projects: &ProjectStore{db},
		Users:    &UsersStore{db},
	}
}
