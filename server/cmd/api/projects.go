package main

import (
	"errors"
	"log"
	"net/http"

	"github.com/db-keli/shinobi/internal/store"
	"github.com/db-keli/shinobi/internal/validator"
)

func (api *application) showProjectHandler(w http.ResponseWriter, r *http.Request) {
	id, err := api.readIDParam(r)
	log.Printf("Parsed ID: %d", id)
	if err != nil {
		api.notFoundResponse(w, r)
		return
	}

	project, err := api.store.Project.Get(id)
	if err != nil {
		switch {
		case errors.Is(err, store.ErrRecordNotFound):
			api.notFoundResponse(w, r)
		default:
			api.serverErrorResponse(w, r, err)
		}
		return
	}

	err = api.writeJSON(w, http.StatusOK, envelope{"project": project}, nil)
	if err != nil {
		api.serverErrorResponse(w, r, err)
	}
}

func (api *application) createProjectHandler(w http.ResponseWriter, r *http.Request) {
	var input = store.ProjectInput{}

	err := api.readJSON(w, r, &input)
	if err != nil {
		api.badRequestResponse(w, r, err)
		return
	}

	token, err := input.EncryptKeys([]byte("thisisaverysecurekey1234"))
	if err != nil {
		api.serverErrorResponse(w, r, err)
		return
	}

	project := &store.Project{
		Name:          input.Name,
		UserID:        1,
		ProjectUrl:    input.ProjectUrl,
		BuildCommands: input.BuildCommands,
		Keystoken:     token,
		ExpireAt:      input.ExpireAt,
	}

	v := validator.New()

	if store.ValidateProject(v, project); !v.Valid() {
		api.failedValidationResponse(w, r, v.Errors)
		return
	}

	err = api.store.Project.Insert(project)
	if err != nil {
		api.serverErrorResponse(w, r, err)
		return
	}

	err = api.writeJSON(w, http.StatusCreated, envelope{"project": project}, nil)
	if err != nil {
		api.serverErrorResponse(w, r, err)
		return
	}
}

func (api *application) deleteProjectHandler(w http.ResponseWriter, r *http.Request) {
	id, err := api.readIDParam(r)
	log.Printf("Attempting to delete project with ID: %d", id)
	if err != nil {
		log.Printf("Error parsing ID: %v", err)
		api.notFoundResponse(w, r)
		return
	}

	err = api.store.Project.Delete(id)
	if err != nil {
		switch {
		case errors.Is(err, store.ErrRecordNotFound):
			api.notFoundResponse(w, r)
		default:
			api.serverErrorResponse(w, r, err)
		}
		return
	}

	log.Printf("Successfully deleted project with ID: %d", id)
	w.WriteHeader(http.StatusNoContent)
}
