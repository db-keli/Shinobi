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

```
├── cmd/                          # Entry points for different application components
├── api/                          # Contains main application logic and route handlers
│   ├── api.go                    # Sets up API routes and server structure
│   ├── context.go                # Manages context handling for API requests
│   ├── errors.go                 # Custom error definitions and handling logic
│   ├── health.go                 # Health check endpoint implementation
│   ├── helpers.go                # Utility functions for various code sections
│   ├── main.go                   # Main entry point for starting the server
│   ├── middleware.go             # Middleware for request handling, like auth and logging
│   ├── project_allowed_users.go  # Manages allowed users for specific projects
│   ├── projects.go               # CRUD operations for projects
│   ├── tokens.go                 # Handles authentication tokens for users
│   └── users.go                  # User registration, authentication, and data management
├── migrate/
│   └── migrations/               # Database migration scripts for schema changes
├── docs/                         # Internal documentation, excluding Swagger files
├── internal/                     # Core components not meant for public exposure
│   ├── db/                       # Database logic, including connection handling
│   ├── env/                      # Environment variable management
│   ├── jsonlog/                  # Structured JSON logging for the server
│   ├── qrcode-generator/         # QR code generation functionality
│   └── store/                    # Data access layers for entities like users, projects
├── validator/                    # Data validation to ensure input meets requirements
├── README.md                     # Project overview, setup instructions, and usage
└── scripts/
    └── db_init.sql               # SQL script for initializing the database schema
```


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
  "project_name": "<project_name>",
  "user_email": "<user_email>"
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
		"token": "DU4SSHOV3W3VCHWZXZQFCLWWPE",
		"expiry": "2024-10-28T14:24:27.716633595Z"
	}
}
