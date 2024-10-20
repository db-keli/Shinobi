package main

import (
	"log"
	"net/http"

	"github.com/db-keli/shinobi/internal/store"
)

func (app *application) AddAllowedUserHandler(w http.ResponseWriter, r *http.Request) {
	var input struct {
		ProjectName string `json:"project_name"`
		UserEmail   string `json:"user_email"`
	}

	err := app.readJSON(w, r, &input)
	if err != nil {
		app.badRequestResponse(w, r, err)
		return
	}

	project, err := app.store.Project.GetByName(input.ProjectName)
	if err != nil {
		app.serverErrorResponse(w, r, err)
		return
	}

	user, err := app.store.Users.GetByEmail(input.UserEmail)
	if err != nil {
		app.serverErrorResponse(w, r, err)
		return
	}

	result := app.store.ProjectAllowedUser.AddAllowedUser(project.ID, user.ID)
	if result != nil {
		app.serverErrorResponse(w, r, result)
		return
	}

	err = app.writeJSON(w, http.StatusCreated, envelope{"message": "User added to project"}, nil)
}

func (app *application) RemoveAllowedUserHandler(w http.ResponseWriter, r *http.Request) {
	var input struct {
		ProjectName string `json:"project_name"`
		UserName    string `json:"user_name"`
	}

	err := app.readJSON(w, r, &input)
	if err != nil {
		app.badRequestResponse(w, r, err)
		return
	}

	project, err := app.store.Project.GetByName(input.ProjectName)
	if err != nil {
		app.serverErrorResponse(w, r, err)
		return
	}

	user, err := app.store.Users.GetByEmail(input.UserName)
	if err != nil {
		app.serverErrorResponse(w, r, err)
		return
	}

	result := app.store.ProjectAllowedUser.RemoveAllowedUser(project.ID, user.ID)
	if result != nil {
		app.serverErrorResponse(w, r, result)
		return
	}

	err = app.writeJSON(w, http.StatusOK, envelope{"message": "User removed from project"}, nil)
}

func (app *application) IsUserAllowedHandler(w http.ResponseWriter, r *http.Request) {
	var input struct {
		ProjectName string `json:"project_name"`
		UserName    string `json:"user_name"`
	}

	err := app.readJSON(w, r, &input)
	if err != nil {
		app.badRequestResponse(w, r, err)
		return
	}

	project, err := app.store.Project.GetByName(input.ProjectName)
	if err != nil {
		app.serverErrorResponse(w, r, err)
		return
	}

	user, err := app.store.Users.GetByEmail(input.UserName)
	if err != nil {
		app.serverErrorResponse(w, r, err)
		return
	}

	result, err := app.store.ProjectAllowedUser.IsUserAllowed(project.ID, user.ID)
	if err != nil {
		app.serverErrorResponse(w, r, err)
		return
	}

	err = app.writeJSON(w, http.StatusOK, envelope{"is_allowed": result}, nil)
}

func (app *application) getKeysHandler(w http.ResponseWriter, r *http.Request) {
	var input struct {
		ProjectName string `json:"project_name"`
		Token       string `json:"token"`
	}

	err := app.readJSON(w, r, &input)
	if err != nil {
		app.badRequestResponse(w, r, err)
		return
	}

	log.Print(input.Token)

	project, err := app.store.Project.GetByName(input.ProjectName)
	if err != nil {
		app.serverErrorResponse(w, r, err)
		return
	}
	user := app.contextGetUser(r)

	result, err := app.store.ProjectAllowedUser.IsUserAllowed(project.ID, user.ID)
	if err != nil {
		app.serverErrorResponse(w, r, err)
		return
	}

	if result == false {
		app.notPermittedRespose(w, r)
		return
	}

	var TempProject store.ProjectInput = store.ProjectInput{
		Name: project.Name,
	}

	key := []byte("thisisaverysecurekey1234")

	err = TempProject.DecryptKeys(input.Token, key)
	if err != nil {
		app.notPermittedRespose(w, r)
		return
	}

	ProjectKeys := TempProject.Keys

	err = app.writeJSON(w, http.StatusOK, envelope{"keys": ProjectKeys}, nil)
}
