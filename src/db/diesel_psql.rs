extern crate diesel;
extern crate dotenv;

use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

// Schema Definitions
table! {
    sessions (session_id) {
        session_id -> Varchar,
        session_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    datasets (dataset_id) {
        dataset_id -> Int4,
        session_id -> Varchar,
        dataset_name -> Varchar,
        data_path -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    models (model_id) {
        model_id -> Int4,
        session_id -> Varchar,
        model_name -> Varchar,
        model_type -> Varchar,
        model_params -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    results (result_id) {
        result_id -> Int4,
        model_id -> Int4,
        session_id -> Varchar,
        accuracy -> Float4,
        precision -> Float4,
        recall -> Float4,
        f1_score -> Float4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

// Model Structs
#[derive(Queryable, Insertable, Debug)]
#[table_name = "sessions"]
pub struct Session {
    pub session_id: String,
    pub session_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "datasets"]
pub struct Dataset {
    pub dataset_id: i32,
    pub session_id: String,
    pub dataset_name: String,
    pub data_path: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "models"]
pub struct Model {
    pub model_id: i32,
    pub session_id: String,
    pub model_name: String,
    pub model_type: String,
    pub model_params: serde_json::Value,  // For model parameters in JSON
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "results"]
pub struct Result {
    pub result_id: i32,
    pub model_id: i32,
    pub session_id: String,
    pub accuracy: f32,
    pub precision: f32,
    pub recall: f32,
    pub f1_score: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Database Connection Setup
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// CRUD Operations

// Create a new session
pub fn create_session(conn: &PgConnection, session_name: &str) -> String {
    use self::sessions::dsl::*;

    let new_session = Session {
        session_id: Uuid::new_v4().to_string(),
        session_name: session_name.to_string(),
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(sessions)
        .values(&new_session)
        .execute(conn)
        .expect("Error creating new session");

    new_session.session_id
}

// Add a dataset
pub fn add_dataset(conn: &PgConnection, session_id_val: &str, dataset_name_val: &str, data_path_val: &str) {
    use self::datasets::dsl::*;

    let new_dataset = Dataset {
        dataset_id: 0, // This will be auto-incremented
        session_id: session_id_val.to_string(),
        dataset_name: dataset_name_val.to_string(),
        data_path: data_path_val.to_string(),
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(datasets)
        .values(&new_dataset)
        .execute(conn)
        .expect("Error adding dataset");
}

// Add a model
pub fn add_model(conn: &PgConnection, session_id_val: &str, model_name_val: &str, model_type_val: &str, model_params_val: serde_json::Value) {
    use self::models::dsl::*;

    let new_model = Model {
        model_id: 0, // This will be auto-incremented
        session_id: session_id_val.to_string(),
        model_name: model_name_val.to_string(),
        model_type: model_type_val.to_string(),
        model_params: model_params_val,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(models)
        .values(&new_model)
        .execute(conn)
        .expect("Error adding model");
}

// Fetch all sessions
pub fn fetch_sessions(conn: &PgConnection) -> Vec<Session> {
    use self::sessions::dsl::*;

    sessions
        .load::<Session>(conn)
        .expect("Error loading sessions")
}

fn main() {
    let conn = establish_connection();
    let session_id = create_session(&conn, "My First Rust Session");
    println!("Created session with ID: {}", session_id);

    add_dataset(&conn, &session_id, "Iris Dataset", "/path/to/iris.csv");
    println!("Added dataset to session.");

    let model_params = serde_json::json!({ "n_estimators": 100, "max_depth": 5 });
    add_model(&conn, &session_id, "Random Forest Classifier", "RandomForest", model_params);
    println!("Added model to session.");

    let sessions = fetch_sessions(&conn);
    println!("Sessions: {:?}", sessions);
}
