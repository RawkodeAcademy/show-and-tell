# EdgeDB

Showcase of EdgeDB

## Testing

* `./setup.sh` to init project and run migrations
* `[GITHUB_TOKEN=<personal github token>] cargo run`
* `curl -XPOST http://localhost:1337/repository/rawkode` to add all starred repos of `rawkode` to the database
  * `curl -XPOST http://localhost:1337/user/<user_name>`
* `curl http://localhost:1337/repository` to get all repositories
* `curl http://localhost:1337/repository/by/language` to get all repositories grouped by language
* `./teardown.sh` to drop the database

## Useful tooling

* `./check.sh` to fmt, lint, and test the project
