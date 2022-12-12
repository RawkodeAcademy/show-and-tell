# EdgeDB vs SurrealDB

## Usecase

Add these endpoints:

* `POST /user/<user_name>` - add all starred repos of the user to the database
* `GET /repository` - list all repos
* `GET /repository/by/language` - list all repos grouped by language

Use the "Languages" of each repo to add tags and store them in a database.
Create a small web app to show the repos based on the tags.

One project uses EdgeDB, the other uses SurrealDB to store the data.
