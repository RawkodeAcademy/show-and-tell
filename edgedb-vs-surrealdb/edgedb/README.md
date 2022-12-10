# EdgeDB

Showcase of EdgeDB

## Testing

* `./setup.sh` to init project and run migrations
* `PERSONAL_GITHUB_TOKEN=<personal github token> cargo run`
* `curl -XPOST http://localhost:1337/repository/comtrya/comtrya` to add `comtrya` to the database
  * `curl -XPOST http://localhost:1337/repository/<owner>/<repo>`
* `curl http://localhost:1337/repository` to get all repositories
* `curl http://localhost:1337/repository/by/language` to get all repositories grouped by language
* `./teardown.sh` to drop the database

## Useful tooling

* `./check.sh` to fmt, lint, and test the project
