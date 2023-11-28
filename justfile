set dotenv-load

_default:
  @just --list

watch-sql:
  watchexec --exts liquid -- sqlweld -v

update-json-schema:
  cd dev-utils && cargo run --bin write_schema > ../schema/app_data.json
  cd platform/app-js && pnpm run update-json-schema && pnpm run build

setup-db:
  cd platform/core && sqlx database setup
