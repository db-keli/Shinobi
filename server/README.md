#### Shinobi Server

###### Endpoints
| **Route**                          | **HTTP Method** | **Handler**                        | **Description**                                                                                         |
|------------------------------------|----------------|----------------------------------|---------------------------------------------------------------------------------------------------------|
| `/v1/health`                       | `GET`          | `healthCheckHandler`              | Checks the health of the service and requires the user to be authenticated.                             |
| `/v1/projects`                     | `POST`         | `createProjectHandler`            | Creates a new project with the given details.                                                           |
| `/v1/projects/{id}`                | `GET`          | `showProjectHandler`              | Retrieves a specific project by its ID.                                                                 |
| `/v1/projects/{id}`                | `DELETE`       | `deleteProjectHandler`            | Deletes a specific project by its ID.                                                                   |
| `/v1/projects/{name}/qrcode`       | `GET`          | `createQRCodeHandler`             | Generates a QR code for a specific project based on its name.                                           |
| `/v1/auth/token`                   | `POST`         | `createAuthenticationTokenHandler`| Creates an authentication token for the user based on their email and password.                         |
| `/v1/auth/register`                | `POST`         | `registerUserHandler`             | Registers a new user with the given name, email, and password.                                          |
| `/v1/projects/add-user`            | `POST`         | `AddAllowedUserHandler`           | Adds a user to the list of allowed users for a specific project.                                        |
| `/v1/projects/remove-user`         | `POST`         | `RemoveAllowedUserHandler`        | Removes a user from the list of allowed users for a specific project.                                   |
| `/v1/projects/check-user`          | `POST`         | `IsUserAllowedHandler`            | Checks if a user is allowed access to a specific project.                                               |
| `/v1/projects/get-keys`            | `POST`         | `getKeysHandler`                  | Retrieves encrypted keys for a project if the user is authorized.                                       |

###### Directory Structure

| **Directory/File**            | **Description**                                                                                                      |
|-------------------------------|----------------------------------------------------------------------------------------------------------------------|
| **`cmd/`**                    | Holds the entry points for different components of the application.                                                   |
| `api/`                        | Contains all the API-related code, including the main application logic and route handlers.                           |
| `api.go`                      | Sets up the API routes and defines the overall structure of the server.                                               |
| `context.go`                  | Manages context handling for API requests, useful for passing data through the request chain.                        |
| `errors.go`                   | Contains custom error definitions and handling logic.                                                                |
| `health.go`                   | Provides the health check endpoint implementation.                                                                   |
| `helpers.go`                  | Contains utility functions that assist various parts of the code.                                                     |
| `main.go`                     | The main entry point for starting the server.                                                                         |
| `middleware.go`               | Defines middleware functions for request handling, such as authentication and logging.                               |
| `project_allowed_users.go`    | Manages logic related to users allowed to access specific projects.                                                  |
| `projects.go`                 | Handles CRUD operations for projects.                                                                                 |
| `tokens.go`                   | Deals with authentication tokens for users.                                                                           |
| `users.go`                    | Manages user registration, authentication, and user data.                                                             |
| `migrate/migrations/`         | Contains database migration scripts that manage schema changes.                                                      |
| **`docs/`**                   | Documentation files, focusing on internal project guidelines or API details (excluding Swagger files).               |
| **`internal/`**               | Contains core components of the application that are not intended to be exposed publicly.                            |
| `db/`                         | Contains database-related logic, including connection handling.                                                      |
| `env/`                        | Handles environment variable management for the application.                                                         |
| `jsonlog/`                    | Implements structured JSON logging for the server.                                                                   |
| `qrcode-generator/`           | Manages QR code generation functionality.                                                                             |
| `store/`                      | Includes data access layers for different entities like projects, users, tokens, and project permissions.            |
| `validator/`                  | Implements data validation logic to ensure that the input meets the required criteria.                               |
| **`README.md`**               | The main readme file that provides a project overview, setup instructions, and usage guidelines.                     |
| **`scripts/`**                | Contains SQL scripts used for database setup and initialization.                                                     |
| `db_init.sql`                 | SQL script for initializing the database schema.                                                                     |

Here's the Markdown file with the commands ready for copy-pasting.

---

# Shinobi API Documentation

**Domain**: `https://shinobi.up.railway.app`

## Table of Contents

- [Health Check](#health-check)
- [Project Endpoints](#project-endpoints)
- [User Endpoints](#user-endpoints)
- [Authentication Endpoints](#authentication-endpoints)
- [Swagger Documentation](#swagger-documentation)

---

## Health Check

```bash
curl -X GET "https://shinobi.up.railway.app/health"
```

---

## Project Endpoints

### Show Project Details by ID

```bash
curl -X GET "https://shinobi.up.railway.app/projects/show/{id}" -H "Authorization: Bearer <token>"
```

### Delete Project by ID

```bash
curl -X GET "https://shinobi.up.railway.app/projects/delete/{id}" -H "Authorization: Bearer <token>"
```

### Create New Project

```bash
curl -X POST "https://shinobi.up.railway.app/projects/create" -H "Authorization: Bearer <token>" -d '{
  "project_name": "<project_name>",
  "description": "<description>"
}'
```

### Generate QR Code for Project by Name

```bash
curl -X GET "https://shinobi.up.railway.app/projects/getQRCode/{name}" -H "Authorization: Bearer <token>"
```

### Retrieve Project Keys

```bash
curl -X POST "https://shinobi.up.railway.app/projects/getkeys" -H "Authorization: Bearer <token>" -d '{
  "project_id": "<project_id>"
}'
```

### Add Allowed User to Project

```bash
curl -X POST "https://shinobi.up.railway.app/projects/allow" -H "Authorization: Bearer <token>" -d '{
  "project_id": "<project_id>",
  "user_id": "<user_id>"
}'
```

### Remove Allowed User from Project

```bash
curl -X POST "https://shinobi.up.railway.app/projects/deny" -H "Authorization: Bearer <token>" -d '{
  "project_id": "<project_id>",
  "user_id": "<user_id>"
}'
```

---

## User Endpoints

### Register New User

```bash
curl -X POST "https://shinobi.up.railway.app/users/register" -d '{
  "name": "<name>",
  "email": "<email>",
  "password": "<password>"
}'
```

---

## Authentication Endpoints

### Generate Authentication Token

```bash
curl -X POST "https://shinobi.up.railway.app/auth/token" -d '{
  "email": "<email>",
  "password": "<password>"
}'
```

---

## Swagger Documentation

### Access API Documentation

```bash
curl -X GET "https://shinobi.up.railway.app/swagger/doc.json"
```

---

{
	"authentication_token": {
		"token": "6A5WARRAENHCEC3J3KCXCNWLY4",
		"expiry": "2024-10-28T14:24:27.716633595Z"
	}
}
