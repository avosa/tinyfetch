use reqwest::{Client, Method, RequestBuilder, StatusCode};
use std::io::{self};

pub struct TinyFetch {}

impl TinyFetch {
    fn request_builder(url: &str, method: Method) -> RequestBuilder {
        Client::new().request(method, url)
    }

    pub async fn get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = Self::request_builder(url, Method::GET).send().await?;

        if response.status() == StatusCode::OK {
            let body = response.text().await?;
            Ok(body)
        } else {
            Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get the response body.",
            )))
        }
    }

    pub async fn post(url: &str, body: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = Self::request_builder(url, Method::POST)
            .body(body.to_owned())
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            let body = response.text().await?;
            Ok(body)
        } else {
            Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get the response body.",
            )))
        }
    }

    pub async fn put(url: &str, body: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = Self::request_builder(url, Method::PUT)
            .body(body.to_owned())
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            let body = response.text().await?;
            Ok(body)
        } else {
            Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get the response body.",
            )))
        }
    }

    pub async fn delete(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = Self::request_builder(url, Method::DELETE).send().await?;

        if response.status() == StatusCode::OK {
            let body = response.text().await?;
            Ok(body)
        } else {
            Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get the response body.",
            )))
        }
    }

    pub async fn patch(url: &str, body: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = Self::request_builder(url, Method::PATCH)
            .body(body.to_owned())
            .send()
            .await?;

        if response.status() == StatusCode::OK {
            let body = response.text().await?;
            Ok(body)
        } else {
            Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Failed to get the response body.",
            )))
        }
    }
}