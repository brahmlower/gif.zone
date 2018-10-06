# Gif Zone

## Development dependencies

- cargo: For building the Rust backend
- npm: For building the React frontend
- sqitch: For database migrations
- docker: Application packaging & delivery
- docker-compose
- openssl

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

Before starting the frontend, you will need to generate self signed certificates for the test site. The `gen_tls_keys.sh` file in the dev_config folder will generate the required resources. A quality of life make target `gen-keys` is available for calling this script. The docker-compose file is pointed to these keys by default.

```
make gen-keys
make frontend-build
make db-start
make db-deploy
docker-compose up
```

The local instance will be available at https://test.gif.zone:8443. The API is proxied at https://test.gif.zone:8443/api. You will want to create an entry in your hosts file (`/etc/hosts`) for this:

```
127.0.0.1 test.gif.zone
```

The nginx config is set up to redirect all HTTP traffic on port 80 to HTTPS. The nginx config is unaware of the port forwards created by docker. This essentially means the HTTP->HTTPS redirect is broken. You will need to be sure you are using the correct protocol and port number (https, 8443).

Also note that if you use Postman to test the full stack locally, it's default configuration is set to disregard self-signed certificates. Fix this by turning off 'SSL certificate verification' in Settings > General.
