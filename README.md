# API template

## Working with repo

We use `Make` as a primary build tool. Here are the most important commands,
others you will find in `Makefile`.

## Requirements

Setup AWS DynamoDB locally: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DynamoDBLocal.html

After install DynamoDB locally, to access DynamoDB running locally with the AWS CLI, use the --endpoint-url parameter. For example, use the following command to list DynamoDB tables.

```
aws dynamodb list-tables --endpoint-url http://localhost:8000
```

#### `make build`

Installs dependencies and build it.

#### `make docker-build`

Builds a Docker image for the service.

#### `make test`

Runs all the test suits: API, integration and unit tests.
