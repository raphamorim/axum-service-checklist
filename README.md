# API template

## Working with repo

We use `Make` as a primary build tool. Here are the most important commands,
others you will find in `Makefile`.

#### `make build`

Installs dependencies and build it.

#### `make docker-build`

Builds a Docker image for the service.

#### `make test`

Runs all the test suits: API, integration and unit tests.
