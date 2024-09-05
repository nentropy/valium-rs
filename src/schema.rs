// src/schema.rs

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    input_validations (id) {
        id -> Int4,
        pattern -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    output_validations (id) {
        id -> Int4,
        pattern -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::*;

    validation_runs (id) {
        id -> Int4,
        validation_type -> Varchar,
        run_time -> Timestamp,
        status -> Varchar,
        errors_found -> Int4,
    }
}

joinable!(output_validations -> validation_runs(id));
joinable!(input_validations -> validation_runs(id));

allow_tables_to_appear_in_same_query!(
    input_validations,
    output_validations,
    validation_runs
);
