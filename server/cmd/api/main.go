package main

import (
	"log"

	_ "github.com/lib/pq"

	_ "github.com/db-keli/shinobi/docs"
	"github.com/db-keli/shinobi/internal/db"
	"github.com/db-keli/shinobi/internal/env"
	"github.com/db-keli/shinobi/internal/store"
)

func main() {
	// err := godotenv.Load(".env")
	// if err != nil {
	// 	log.Fatalf("Error loading .envrc file: %v", err)
	// }

	cfg := config{
		addr: env.GetString("ADDR", "0.0.0.0:") + env.GetString("PORT", "8080"),
		db: dbConfig{
			addr:         env.GetString("DB_ADDR", "postgres://admin:adminpassword@localhost:5543/shinobi2-db?sslmode=disable"),
			maxOpenConns: env.GetInt("DB_MAX_OPEN_CONNS", 5),
			maxIdleConns: env.GetInt("DB_MAX_IDLE_CONNS", 2),
			MaxIdleTime:  env.GetString("DB_MAX_IDLE_TIME", "15m"),
		},
	}

	db, err := db.New(cfg.db.addr, cfg.db.maxOpenConns, cfg.db.maxIdleConns, cfg.db.MaxIdleTime)

	if err != nil {
		log.Fatalf("db.New: %v", err)
	}

	store := store.NewPostgresStorage(db)

	app := application{
		config: cfg,
		store:  store,
	}

	mux := app.mount()

	log.Fatal(app.run(mux))
}
