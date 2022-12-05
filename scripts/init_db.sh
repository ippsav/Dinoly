#!/bin/bash

set -x
set -eo pipefail



if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

DB_USER="${POSTGRES_USER:=user}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_HOST="${POSTGRES_HOST:=localhost}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_NAME="${POSTGRES_DB:=dinoly-db}"



if [[ -z "${SKIP_DOCKER}" ]]
then
  RUNNING_POSTGRES_CONTAINER=$(docker ps --filter 'name=dinoly' --format '{{.ID}}')
  if [[ -n $RUNNING_POSTGRES_CONTAINER ]]; then
    echo >&2 "there is a postgres instance running in the background kill it with"
    echo >&2 " docker kill ${RUNNING_POSTGRES_CONTAINER}"
    exit 1
  fi

  docker run \
    -e POSTGRES_USER=${DB_USER}\
    -e POSTGRES_PASSWORD=${DB_PASSWORD}\
    -e POSTGRES_DB=${DB_NAME}\
    -e POSTGRES_HOST=${DB_HOST}\
    -p "${DB_PORT}":5432\
    -d \
    --name "dinoly"\
    postgres -N 1000
fi




until PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable -sleeping"
  sleep 1
done

>&2 echo "Postgres is running on port ${DB_PORT} - running migrations now !"

DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"

cd migration && cargo run up -u ${DATABASE_URL}

>&2 echo "Postgres has been migrated, ready to go!"

