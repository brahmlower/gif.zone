# Gif Zone

## Development dependencies

- cargo: For building the Rust backend
- npm: For building the React frontend
- sqitch: For database migrations
- docker: Application packaging & delivery
- docker-compose

## Backend

The gif.zone backend is written in Rust. Compiling and running is as simple as:

```
make backend-build
make backend-run
```

## Frontend

The gif.zone frontend is a single page react app.

```
make frontend-setup
make frontend-build
```

## Local stack

This assumes you have docker and docker-compose installed on your local machine. You will need to build the project before running docker-compose, since the frontend container is just an nginx container with `frontend/build` mounted as a volume.

```
make db-start
make db-deploy
make frontend-build
docker-compose up
```
