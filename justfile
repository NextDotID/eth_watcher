DEFAULT_ENV := "DATABASE_URL=postgresql://ethwatcher:eiyaeDiephah0xoh2Aelaikae0phuuth@localhost:5431/ethwatcher"
ENV_EXISTS := path_exists("./.env")

[private]
default:
    @just --list

[private]
@prepare_env:
    if not {{ENV_EXISTS}}; then echo "Creating default environment file..."; echo {{DEFAULT_ENV}} > .env; fi

[private]
@generate-entity:
    rm -r ./entity/src
    sea-orm-cli generate entity -v --lib --expanded-format -o ./entity/src

# Prepare development envrionment.
@prepare: && prepare_env
    cargo install sea-orm-cli
    docker compose up -d

# Generate a new migration change to edit.
@generate-migration NEW_MIGRATION_NAME:
    sea-orm-cli migrate generate -d ./entity {{NEW_MIGRATION_NAME}}

# Apply all migration changes to DB and regenerate entity definition.
@migrate: && generate-entity
    sea-orm-cli migrate

@rollback: && generate-entity
    sea-orm-cli migrate down
