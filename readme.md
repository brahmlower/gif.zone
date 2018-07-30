# Gif Zone

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