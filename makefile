
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

.PHONY: frontend-run
frontend-run:
	cd frontend && serve -s build

.PHONY: frontend-clean
frontend-clean:
	rm -rf frontend/build

# Docker -----------------------------------------------------------------------

.PHONY: frontend-docker-build
frontend-docker-build:
	docker-compose build frontend

.PHONY: frontend-docker-run
frontend-docker-run:
	docker-compose up web

# Deployment -------------------------------------------------------------------

.PHONY: frontend-aws-setup
frontend-aws-setup:
	virtualenv -p python3.6 venv
	source venv/bin/activate && pip install awscli
	@echo "\n\nRun 'source venv/bin/activate' to use the 'aws' utility.\n\n"

.PHONY: frontend-aws-push
frontend-aws-push:
	docker build -t gif_zone .
	docker tag gif_zone:latest 918505329056.dkr.ecr.us-west-2.amazonaws.com/testing-gif_zone:latest
	docker push 918505329056.dkr.ecr.us-west-2.amazonaws.com/testing-gif_zone:latest
