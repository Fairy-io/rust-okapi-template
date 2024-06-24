#!/bin/bash

OS="$(uname -s)"
OS_TYPE="not_mac"
MODE=$1

case "${OS}" in
    Darwin*)    OS_TYPE="mac";;
    *)          OS_TYPE="not_mac"
esac

load_envs() {
    local env_file
    
    if [ "$MODE" == "test" ]; then
        env_file=".env.test"
    else
        env_file=".env"
    fi

    if [ -f "test.db" ]; then
        export $(grep -v "^#" "$env_file" | xargs)
    fi
}

clear_empty_migrations() {
    local migrations_dir=$1

    find "$migrations_dir" -type d -name "*_migration" | while read -r subfolder; do
    content_file="$subfolder/up.sql"
    
        if [ -f "$content_file" ]; then
            content=$(<"$content_file")
            empty_migration=$(<"./sqlite/empty_migration")

            if [ "$content" == "$empty_migration" ]; then
                rm -rf "$subfolder"
            fi
        fi
    done
}

prepare_database() {
    local config_file
    local migrations_dir

    if [ "$MODE" == "test" ]; then
        config_file="diesel.test.toml"
        migrations_dir="migrations_test"
    else
        config_file="diesel.toml"
        migrations_dir="migrations"
    fi

    if [ "$MODE" == "test" ] && [ ! -f "test.db" ]; then
        if [ "$OS_TYPE" == "mac" ]; then
            ./sqlite/sqlite3_mac test.db "VACUUM;"
        else
            ./sqlite/sqlite3_linux test.db "VACUUM;"
        fi
    fi

    diesel migration --config-file "$config_file" generate --diff-schema migration
    clear_empty_migrations "$migrations_dir"

    diesel migration --config-file "$config_file" --locked-schema run
    find "$migrations_dir" -type d -name "*_migration" -exec rm -r {} +
}

load_envs
prepare_database