set dotenv-load

_default:
  @just --list

watch-sql:
  watchexec --exts tera -- just sqlweld

sqlweld:
  sqlweld -v --formatter pg_format

update-json-schema:
  cd dev-utils && cargo run --bin write_schema > ../schema/app_data.json
  cd platform/app-js && pnpm run update-json-schema && pnpm run build

setup-db:
  cd platform/core && sqlx database setup

dev-api:
  cd ../filigree/filigree-cli && cargo build
  cd platform/core && ../../../filigree/target/debug/filigree && GLANCE_LOG=debug cargo run --release -- serve

dev-web:
  #cd ../filigree/web && pnpm package
  # cd platform/web && pnpm i
  cd platform/web && pnpm dev
