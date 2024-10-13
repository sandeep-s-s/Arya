use crate::{db, models, schema};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::*;

#[tauri::command]
pub fn create_collection(name: String) -> Collection {
    use crate::schema::*;
    let connection = &mut db::establish_connection();
    let uuid = Uuid::new_v4().hyphenated().to_string();

    let new_collection = NewCollection {
        name: name,
        uuid: String::from(&uuid),
    };
    diesel::insert_into(collections::table)
        .values(&new_collection)
        .returning(Collection::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}

#[tauri::command]
pub async fn get_collections() -> Vec<CollectionWithRequests> {
    let mut connection = db::establish_connection();

    let all_collections: Vec<Collection> = schema::collections::table
        .select(Collection::as_select())
        .load(&mut connection)
        .expect("Error in fetching all collections");

    // Load all requests
    let all_requests: Vec<Requests> = schema::requests::table
        .select(Requests::as_select())
        .load(&mut connection)
        .expect("Error in fetching all requests");

    // Group the requests per collection
    let requests_per_collection = all_requests
        .grouped_by(&all_collections)
        .into_iter()
        .zip(all_collections)
        .map(|(requests, collection)| CollectionWithRequests {
            collection,
            requests,
        })
        .collect::<Vec<CollectionWithRequests>>();

    requests_per_collection
}

#[tauri::command]
pub fn create_request(name: String, uuid: String) -> Requests {
    use crate::schema::*;
    let connection = &mut db::establish_connection();
    let request_uuid = Uuid::new_v4().hyphenated().to_string();

    let collection: models::Collection = schema::collections::table
        .filter(schema::collections::uuid.eq(uuid)) // Add filter for uuid
        .select(Collection::as_select())
        .first(connection) // Load the first matching record
        .expect("Error loading collection"); //

    let new_request: NewRequest = NewRequest {
        name: name,
        request_data: "{}".to_string(),
        uuid: String::from(&request_uuid),
        collection_id: collection.id,
    };
    diesel::insert_into(requests::table)
        .values(&new_request)
        .returning(Requests::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}


