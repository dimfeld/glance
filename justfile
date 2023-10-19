_default:
  @just --list

update-json-schema:
  cargo typify -o platform/glance-types-rust/src/app_data.rs schema/app_data.json
  cd platform/glance-types-js && pnpm run update-json-schema
