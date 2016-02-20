
.PHONY: debug all build postgres

all:
	docker pull postgres:9.5
	cargo build --release

debug:
	cargo build

postgres:
	docker run --name session-postgres -e POSTGRES_PASSWORD=mysecretpassword \
					-e POSTGRES_USER=jonfk \
					-v ./data:/var/lib/postgresql/data \
					-p 5432:5432 \
					-d postgres:9.5
