package main

import (
	"errors"
	"net/http"

	"github.com/db-keli/shinobi/internal/store"
	"github.com/db-keli/shinobi/internal/validator"
)

func (api *application) registerUserHandler(w http.ResponseWriter, r *http.Request) {
	var input struct {
		Name     string `json:"name"`
		Email    string `json:"email"`
		Password string `json:"password"`
	}

	err := api.readJSON(w, r, &input)
	if err != nil {
		api.badRequestResponse(w, r, err)
		return
	}

	user := &store.User{
		Name:      input.Name,
		Email:     input.Email,
		Activated: false,
	}

	err = user.Password.Set(input.Password)
	if err != nil {

		api.serverErrorResponse(w, r, err)
		return
	}

	v := validator.New()

	if store.ValidateUser(v, user); !v.Valid() {
		api.failedValidationResponse(w, r, v.Errors)
		return
	}

	err = api.store.Users.Insert(user)
	if err != nil {
		switch {
		case errors.Is(err, store.ErrDuplicateEmail):
			v.AddError("email", "a user with this email address already exists")
			api.failedValidationResponse(w, r, v.Errors)
		default:
			api.serverErrorResponse(w, r, err)
		}
		return
	}

	err = api.writeJSON(w, http.StatusCreated, envelope{"user": user}, nil)
	if err != nil {
		api.serverErrorResponse(w, r, err)
	}
}
