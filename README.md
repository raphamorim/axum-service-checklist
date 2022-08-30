# Rust service template

## Checklist


- [ ] Serve static files by serving the folder `resources`. Should be able to access `http://localhost:3000/static/great-wave.jpg` and return the image file:

![Demo great wave](resources/demo.png)

- [ ] Implement a route GET:`/user` that parses a local JSON file `user.json` and returns a response with content-type "application/json" and with the body as:

```json
{
	"name": "Emil",
	"age": 7,
	"email": "emil@viaplaygroup.com"
}
```

- [ ] Implement a route GET:`/dog` that makes a request to https://dog.ceo/api/breeds/image/random , parses the response and return the property `message` as:


```json
{
	"dog": "https://images.dog.ceo/breeds/malinois/n02105162_299.jpg"
}
```



## Working with repo

We use `Make` as a primary build tool. Here are the most important commands,
others you will find in `Makefile`.

#### `make build`

Installs dependencies and build it.

#### `make docker-build`

Builds a Docker image for the service.

#### `make test`

Runs all the test suits: API, integration and unit tests.

## Checklist