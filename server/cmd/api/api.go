package main

import (
	"fmt"
	"log"
	"net/http"
	"time"

	_ "github.com/db-keli/shinobi/docs"
	"github.com/db-keli/shinobi/internal/store"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	httpSwagger "github.com/swaggo/http-swagger"
)

type application struct {
	config config
	logger *log.Logger
	store  store.Storage
}
type config struct {
	addr string
	db   dbConfig
}

type dbConfig struct {
	addr         string
	maxOpenConns int
	maxIdleConns int
	MaxIdleTime  string
}

func (app *application) mount() http.Handler {
	r := chi.NewRouter()

	r.Use(middleware.RealIP)
	r.Use(middleware.RequestID)
	r.Use(middleware.Recoverer)
	r.Use(middleware.Logger)

	r.Use(middleware.Timeout(60 * time.Second))

	r.Use(app.authenticate)

	r.Route("/v1", func(r chi.Router) {
		r.Use(app.authenticate)
		r.Get("/health", app.healthCheckHandler)

		//projects
		r.Route("/projects", func(r chi.Router) {
			r.Get("/show/{id}", app.requireAuthenticatedUser(app.showProjectHandler))
			r.Get("/delete/{id}", app.requireAuthenticatedUser(app.deleteProjectHandler))
			r.Post("/create", app.requireAuthenticatedUser(app.createProjectHandler))
			r.Get("/getQRCode/{name}", app.requireAuthenticatedUser(app.createQRCodeHandler))

			//An endpoint that will check client and provide keys
			r.Post("/getkeys", app.requireAuthenticatedUser(app.getKeysHandler))
			r.Post("/allow", app.requireAuthenticatedUser(app.AddAllowedUserHandler))
			r.Post("/deny", app.requireAuthenticatedUser(app.RemoveAllowedUserHandler))

		})

		//users
		r.Route("/users", func(r chi.Router) {
			r.Post("/register", app.registerUserHandler)
		})

		//auth
		r.Route("/auth", func(r chi.Router) {
			r.Post("/token", app.createAuthenticationTokenHandler)
		})

		docsURL := fmt.Sprintf("%s/swagger/doc.json", app.config.addr)
		r.Get("/swagger/*", httpSwagger.Handler(httpSwagger.URL(docsURL)))
	})

	return r
}

func (app *application) run(mux http.Handler) error {

	srv := &http.Server{
		Addr:         app.config.addr,
		Handler:      mux,
		WriteTimeout: time.Second * 30,
		ReadTimeout:  time.Second * 10,
		IdleTimeout:  time.Minute,
	}

	log.Println("Starting server on", app.config.addr)

	return srv.ListenAndServe()
}
