
.PHONY: debug all build postgres

all:
	docker pull postgres:9.5
	cargo build --release

debug:
	cargo build

postgres:
	# Commented out on osx because shared volumes with osx do not work
	# -v "$(shell pwd)/postgresdata":/var/lib/postgresql/data
	docker run --name session-postgres -e POSTGRES_PASSWORD=mysecretpassword \
					-e POSTGRES_USER=jonfk \
					-p 5432:5432 \
					-d postgres:9.5

elastic:
	# Commented out on osx because shared volumes with osx do not work
	# -v "$(shell pwd)/esdata":/usr/share/elasticsearch/data
	docker run --name session-elastic \
					-p 9200:9200 \
					-p 9300:9300 \
					-d elasticsearch elasticsearch -Des.node.name="TestNode"
