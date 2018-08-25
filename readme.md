# Gif Zone

## Build dependencies

- cargo
- npm
- docker
- docker-compose

## Running Locally

Everything is set up through docker and docker compose. After cloning the repo
you'll want to build the frontend (cleanroom builds aren't set up yet) and then
build the docker images:

```
make frontend-setup
make frontend-build
docker-compose build
```

## Local development

Build and run locally

```
make build
make run
```

## Docker

Build and run the docker container locally

```
make docker-build
make docker-run
```

## Deployment

You will need the aws cli, which is installed via pip. To avoid polluting your
local machine, it's installed in a virtualenv. This process is wrapped in a
make target:

```
make aws-setup
source venv/bin/activate
```

From there, if you haven't already configured the api keys, you will need to do
so. Adequate instruction on this process are found here, though you can run the
following command which will prompt for each value. We are using `us-west-2`
(oregon).

```
aws configure
```

https://us-west-2.console.aws.amazon.com/ecs/home?region=us-west-2#/clusters