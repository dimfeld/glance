_default:
  @just --list

update-json-schema:
  cargo typify -o platform/glance-app-rust/src/app_data.rs -a PartialEq -a Eq schema/app_data.json
  cd platform/glance-app-js && pnpm run update-json-schema
