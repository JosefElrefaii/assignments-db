## Quickstart:

- Have Rust installed (https://rustup.rs)
- Have Docker installer (https://docs.docker.com/get-docker/)
- Do the "Database Setup"

## Making changes to the database schema

If you do this, please run:

```
cargo sqlx prepare
```

And commit the `sqlx-data.json`. This is needed for CI.

## Database Setup

Change the environment variables to your liking, but note that `DATABASE_URL` MUST be set for `sqlx` or the project won't compile and `sqlx` migrate commands will fail.

```bash
cargo install sqlx-cli --no-default-features --features postgres

export DB_CONTAINER=pg-techno-assignments
export DB_PORT=45432
export DB_NAME=assignments
export DB_USER=assignments
export DB_PASSWORD=assignments
export DATABASE_URL=postgres://$DB_USER:$DB_PASSWORD@localhost:$DB_PORT/$DB_NAME
```

```bash
docker run --name $DB_CONTAINER -e POSTGRES_PASSWORD=postgres -d -p $DB_PORT:5432 postgis/postgis:13-3.1
```

Give Docker some time to spin up the container or the subsequent commands may cause errors.

```bash
PGPASSWORD=postgres createdb -U postgres -h localhost -p $DB_PORT $DB_NAME

PGPASSWORD=postgres psql -U postgres -h localhost -p $DB_PORT $DB_NAME <<EOF
CREATE EXTENSION citext;
CREATE EXTENSION postgis;
CREATE EXTENSION btree_gist;
CREATE EXTENSION "uuid-ossp";
CREATE USER $DB_USER WITH ENCRYPTED PASSWORD '$DB_PASSWORD';
GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;
EOF

sqlx migrate run

PGPASSWORD=$DB_PASSWORD psql -U $DB_NAME -h localhost -p $DB_PORT $DB_NAME < schema/test-data.sql
```

### Managing the Docker Container

```bash
# To start the service again if it stopped:
docker start $DB_CONTAINER

# To stop the service, run:
docker stop $DB_CONTAINER

# To delete it entirely, run (after stopping):
docker rm $DB_CONTAINER
```

### Resetting the Database Schema

This is for early development only, when it is convenient to quickly change the database schema, so this just drops and recreates the database and does a migration.

```bash
PGPASSWORD=postgres dropdb -U postgres -h localhost -p $DB_PORT $DB_NAME
PGPASSWORD=postgres createdb -U postgres -h localhost -p $DB_PORT $DB_NAME

PGPASSWORD=postgres psql -U postgres -h localhost -p $DB_PORT $DB_NAME <<EOF
CREATE EXTENSION citext;
CREATE EXTENSION postgis;
CREATE EXTENSION btree_gist;
CREATE EXTENSION "uuid-ossp";
GRANT ALL PRIVILEGES ON DATABASE $DB_NAME TO $DB_USER;
EOF

sqlx migrate run

PGPASSWORD=$DB_PASSWORD psql -U $DB_USER -h localhost -p $DB_PORT $DB_NAME < schema/test-data.sql
```

### Running Commands

```bash
PGPASSWORD=postgres psql -U postgres -h localhost -p $DB_PORT $DB_NAME <<EOF
SELECT * FROM applications;
EOF
```
