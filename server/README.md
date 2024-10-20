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
