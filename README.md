# HMCTS Task Management Backend

The Task Management system was designed in two seperate repositories, this is the *backend* service, the frontend can be found [here](https://github.com/zeldow010/dts-dev-challenge-frontend).

## Requirements
- `rust 1.86.0` or newer
- Docker

Once installed run `cargo install` to get all packages.

Ensure no other postgres services are running, and the run `./scripts/init_db.sh` which will start the postgres docker container and run the database migration.

On completion of the setup, run `cargo run` to start the backend service.

## Routes

- `GET /health_check` will return a status code of `200 OK` to tell you that the service is live and well.
- `GET /tasks` will display all tasks currently in the database
- `POST /tasks` will add a new task as long it contains a title with a minimum of 3 characters, and a valid ISO date.

```rust
struct task {
    title: String,
    description: Option<String>,
    due_date: Date
}
```
- `GET /tasks/:task_id` will get a specific task as long as it is supplied with a valid `uuid`
- `DELETE /tasks/:task_id` will delete the task with the supplied `task_id`
- `PATCH /tasks/:task_id/next_status` will update the `task_status` of the task using the following enum, where `Completed` is the final update state. This route will always return `200 OK`:
```rust
enum TaskStatus {
    Draft,
    ToDo,
    Completed,
}
```
## Compromises and Improvements

The current state of the service completes all given requirements. Using the `Axum` framework allows to continous operation covering most edgecases as the crate utalises *Rust's Type Safety* to provide validation at route request.

However, in the time I was able to produce this backend service I made a few compromises that would have to be redesigned if the system were to be put into production, mainly:

- Exposing variables and environment variables such as the postgres url.
- Tighted validation and verification on environment variables with appropraite error messages to guide setup.
- Better error messages for client side applications, by designing from a stakeholder perspective.
- Improved Logging and Tracing for external monitoring.

## Beyond the scope

A feature that would need to be added beyond the scope of this challenge would be an authentication and authorisation system.

A new table would need to be added into the database which would contain information about each user. Then the `tasks` table would need to be updated to contain a foreign key to track which task belongs to who.

Of course, authentication system require a lot of attention to ensure safety following **OWASP Best Practices**, like using `argon2` as a cryptographically secure hashing algorithm for passwords.