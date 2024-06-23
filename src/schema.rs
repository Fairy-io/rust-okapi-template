// @generated automatically by Diesel CLI.
// https://diesel.rs/guides/getting-started
// https://docs.diesel.rs/2.2.x/diesel/macro.table.html
// https://diesel.rs/guides/relations.html
// https://github.com/diesel-rs/diesel/issues/2085
// https://stackoverflow.com/questions/59228152/rust-diesel-cli-setting-up-multiple-env-files-for-different-environments

use diesel::*;

table! {
    todos (id) {
        id -> Varchar,
        text -> Text,
        done -> Bool,
        done2 -> Bool,
    }
}
