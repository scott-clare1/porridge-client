use pyo3::prelude::*;
use reqwest::{Client, Request, Response};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use uuid::Uuid;
type Embedding = Vec<f32>;

const SEARCH_ENDPOINT: &str = "/search";
const STORE_ENDPOINT: &str = "/store";
const RETRIEVE_ENDPOINT: &str = "/retrieve";
const HEARTBEAT_ENDPOINT: &str = "/heartbeat";

#[pyclass]
#[derive(Clone, Serialize)]
struct EmbeddingEntry {
    pub embeddings: Embedding,
    pub text: String,
}

#[pyclass]
pub struct PorridgeClient {
    url: String,
    client: Client,
    runtime: Mutex<Runtime>,
}

#[pymethods]
impl PorridgeClient {
    #[new]
    fn new(host: String, port: String) -> Self {
        let url = format!("http://{}:{}", host, port);
        let client = Client::new();
        let runtime = Runtime::new().unwrap();
        Self {
            url,
            client,
            runtime: Mutex::new(runtime),
        }
    }

    fn store(&self, entries: Vec<EmbeddingEntry>) -> PyResult<String> {
        let url = format!("{}{}", self.url, STORE_ENDPOINT);
        let response: Result<String, reqwest::Error> =
            self.runtime.lock().unwrap().block_on(async {
                let response = self.client.post(url).json(&entries).send().await?;
                let body = response.text().await?;
                Ok(body)
            });
        self.handle_response(response)
    }
    fn search(&self, query: EmbeddingEntry) -> PyResult<String> {
        let url = format!("{}{}", self.url, SEARCH_ENDPOINT);
        let response: Result<String, reqwest::Error> =
            self.runtime.lock().unwrap().block_on(async {
                let response = self.client.post(url).json(&query).send().await?;
                let body = response.text().await?;
                Ok(body)
            });
        self.handle_response(response)
    }
    fn retrieve(&self, uuid: String) -> PyResult<String> {
        let url = format!("{}{}/{}", self.url, RETRIEVE_ENDPOINT, uuid);
        let response: Result<String, reqwest::Error> =
            self.runtime.lock().unwrap().block_on(async {
                let response = self.client.get(url).send().await?;
                let body = response.text().await?;
                Ok(body)
            });
        self.handle_response(response)
    }
    fn heartbeat(&self) -> PyResult<String> {
        let url = format!("{}{}", self.url, HEARTBEAT_ENDPOINT);
        let response: Result<String, reqwest::Error> =
            self.runtime.lock().unwrap().block_on(async {
                let response = self.client.get(url).send().await?;
                let body = response.text().await?;
                Ok(body)
            });
        self.handle_response(response)
    }
}

impl PorridgeClient {
    fn handle_response(&self, response: Result<String, reqwest::Error>) -> PyResult<String> {
        match response {
            Ok(body) => Ok(body),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
        }
    }
}

#[pymodule]
fn porridge_client(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PorridgeClient>()?;
    Ok(())
}
