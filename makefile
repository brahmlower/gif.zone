
VERSION=$(shell cat version | head -n 1)
PROJECT_ID=$(shell gcloud config get-value project -q)

# Backend ----------------------------------------------------------------------

.PHONY: backend-build
backend-build:
	cd backend && cargo build

.PHONY: backend-run
backend-run:
	cd backend && cargo run ./config/settings.yml

# Frontend ---------------------------------------------------------------------

.PHONY: frontend-setup
frontend-setup:
	cd frontend && npm install

.PHONY: frontend-build
frontend-build:
	cd frontend && npm run-script build
	cp -r frontend/data frontend/build/data

.PHONY: frontend-lint
frontend-lint:
	cd frontend && ./node_modules/eslint/bin/eslint.js src

.PHONY: frontend-run
frontend-run:
	cd frontend && serve -s build

.PHONY: frontend-clean
frontend-clean:
	rm -rf frontend/build

# Database --------------------------------------------------------------------

.PHONY: db-start
db-start:
	docker-compose up -d db

.PHONY: db-stop
db-stop:
	docker-compose stop db

.PHONY: db-deploy
db-deploy:
	sqitch deploy db:pg://gif_zone:gif_zone@localhost

.PHONY: db-revert
db-revert:
	sqitch revert -y db:pg://gif_zone:gif_zone@localhost

.PHONY: db-shell
db-shell:
	psql -U gif_zone -h localhost gif_zone

# Docker -----------------------------------------------------------------------

.PHONY: backend-docker-build
backend-docker-build:
	docker-compose build backend

.PHONY: backend-docker-run
backend-docker-run:
	docker-compose up backend

.PHONY: frontend-docker-build
frontend-docker-build:
	docker-compose build frontend

.PHONY: frontend-docker-run
frontend-docker-run:
	docker-compose up web

# Deployment -------------------------------------------------------------------

.PHONY: docker-build
docker-build:
	docker build -t gcr.io/${PROJECT_ID}/gif.zone-backend:v${VERSION} -f backend/dockerfile .
	docker build -t gcr.io/${PROJECT_ID}/gif.zone-frontend:v${VERSION} -f frontend/dockerfile .

.PHONY: docker-push
docker-push:
	docker push gcr.io/${PROJECT_ID}/gif.zone-backend:v${VERSION}
	docker push gcr.io/${PROJECT_ID}/gif.zone-frontend:v${VERSION}

.PHONY: kube-deploy
kube-deploy:
	kubectl apply -f infrastructure/gif.zone-deployment.yml
