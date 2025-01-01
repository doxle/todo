use std::u32;

use anyhow::{anyhow, Context, Error, Result};
use chrono::Utc;
use dioxus::logger::tracing;
use rand::{thread_rng, Rng};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TodoType {
    All,
    Active,
    Completed,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub id: u32,
    pub content: String,
    pub checked: bool,
    // pub todo_type: TodoType,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ResponseWrapper {
    pub data: Vec<TodoItem>,
    pub message: String,
    pub status: u32,
}

pub async fn get_all_todos() -> Result<Vec<TodoItem>> {
    let response = reqwest::get("http://localhost:9000/lambda-url/lambda_todo/")
        .await
        .context("Failed to send GET request to the server")?;

    tracing::info!("Response Status: {}", response.status());
    // Extract the response body as text
    let raw_body = response
        .text()
        .await
        .context("Failed to read the response body as text")?;

    let response_wrapper: ResponseWrapper =
        serde_json::from_str(&raw_body).context("Failed to form raw_json")?;

    tracing::info!("Raw Response Body: {:#?}", raw_body);
    tracing::info!("Raw Response WRAPPER: {:#?}", response_wrapper);

    // WE NEED SERDE_JSON FROM_STR TO CONVERT THE SERIALIZED OBJECT INTO RUST TYPES WHICH IS CALLED DESERIALIZATION.
    // SERDE JSON WORKS TOGETHER SERDE DERIVE FEATURE

    let todos = response_wrapper.data;

    Ok(todos)
}

// NEED TIMEBASE UNIQUE ID + RANDOM SUFFIX. USED THIS THAN UUID.V4() AS ATLEAST WE HAVE A TIMESTAMP + UNIQUENESS
pub fn generate_time_based_id() -> u32 {
    let timestamp = Utc::now().timestamp_millis() as u64;
    let random_3_digits = thread_rng().gen_range(0..1000) as u32;
    let truncated_timestamp = (timestamp % 1_000_000) as u32;

    // Combine the truncated timestamp and random number like lexographically with adding them. If we dont multiply
    // by 1000 it will be like 123456 + 789 = 124245 but we want 123456789
    (truncated_timestamp * 1000) + random_3_digits
}

//- ONCE INSERTED, IT RETURNS THE ID
pub async fn add_todo(new_todo: TodoItem) -> Result<(), Error> {
    let client = Client::new();
    tracing::info!("SENDING PUT REQUEST : {:#?}", new_todo);
    let result = client
        .put("http://localhost:9000/lambda-url/lambda_todo/")
        .json(&new_todo)
        .send()
        .await
        .context("Failed to send PUT request")?;

    if result.status().is_success() {
        tracing::info!("{} was successfully added", &new_todo.content);
        Ok(())
    } else {
        tracing::info!("Request failed with status: {}", result.status());
        Err(anyhow!(
            "Request failed with with status:{}",
            result.status()
        ))
    }
}

//- ONCE INSERTED, IT RETURNS STATUS CODE
pub async fn delete_todo(id: u32) -> Result<(), Error> {
    let client = Client::new();
    tracing::info!("SENDING DELEET REQUEST : {:#?}", id);

    let _payload = serde_json::json!({
        "id":id
    });
    let result = client
        .delete("http://localhost:9000/lambda-url/lambda_todo/")
        .body(id.to_string())
        .send()
        .await
        .context("Failed to send PUT request")?;

    if result.status().is_success() {
        tracing::info!("{} was successfully deleted", &id);
        Ok(())
    } else {
        tracing::info!("Request failed with status: {}", result.status());
        Err(anyhow!(
            "Request failed with with status:{}",
            result.status()
        ))
    }
}
