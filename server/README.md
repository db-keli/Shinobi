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
