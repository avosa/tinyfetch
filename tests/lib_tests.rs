use tinyfetch::TinyFetch;
use mockito::{mock, Matcher};

const MOCK_BODY: &str = r#"{
    "id": 1,
    "title": "iPhone 9",
    "description": "An apple mobile which is nothing like apple",
    "price": 549,
    "discountPercentage": 12.96,
    "rating": 4.69,
    "stock": 94,
    "brand": "Apple",
    "category": "smartphones",
    "thumbnail": "https://i.dummyjson.com/data/products/1/thumbnail.jpg",
    "images": [
        "https://i.dummyjson.com/data/products/1/1.jpg",
        "..."
    ]
}"#;

#[tokio::test]
async fn test_get_request() {
    let mock_url = "https://dummyjson.com/products/1";
    let _m = mock("GET", Matcher::Regex(r"^/products/\d+$".to_string()))
        .with_body(MOCK_BODY)
        .create();

    let response = TinyFetch::get(mock_url).await;

    assert!(response.is_ok());
    let body = response.unwrap();
    assert!(body.contains(r#""id":1"#));
    assert!(body.contains(r#""title":"iPhone 9""#));
    assert!(body.contains(r#""description":"An apple mobile which is nothing like apple""#));
    assert!(body.contains(r#""price":549"#));
    assert!(body.contains(r#""discountPercentage":12.96"#));
    assert!(body.contains(r#""rating":4.69"#));
    assert!(body.contains(r#""stock":94"#));
    assert!(body.contains(r#""brand":"Apple""#));
    assert!(body.contains(r#""category":"smartphones""#));
    assert!(body.contains(r#""thumbnail":"https://i.dummyjson.com/data/products/1/thumbnail.jpg""#));
    assert!(body.contains(r#""images":["https://i.dummyjson.com/data/products/1/1.jpg","#));
}

#[tokio::test]
async fn test_patch_request() {
    let mock_url = "https://dummyjson.com/products/1";
    let _m = mock("PATCH", "/products/1")
        .with_body(MOCK_BODY)
        .create();

    let response = TinyFetch::patch(mock_url, "Request body").await;

    assert!(response.is_ok());
    let body = response.unwrap();
    assert!(body.contains(r#""id":1"#));
    assert!(body.contains(r#""title":"iPhone 9""#));
    assert!(body.contains(r#""price":549"#));
    assert!(body.contains(r#""stock":94"#));
    assert!(body.contains(r#""rating":4.69"#));
    assert!(body.contains(r#""images":["https://i.dummyjson.com/data/products/1/1.jpg","#));
    assert!(body.contains(r#""thumbnail":"https://i.dummyjson.com/data/products/1/thumbnail.jpg""#));
    assert!(body.contains(r#""description":"An apple mobile which is nothing like apple""#));
    assert!(body.contains(r#""brand":"Apple""#));
    assert!(body.contains(r#""category":"smartphones""#));
}

#[tokio::test]
async fn test_put_request() {
    let mock_url = "https://dummyjson.com/products/1";
    let _m = mock("PATCH", "/products/1")
        .with_body(MOCK_BODY)
        .create();

    let response = TinyFetch::put(mock_url, "Request body").await;

    assert!(response.is_ok());
    let body = response.unwrap();
    assert!(body.contains(r#""id":1"#));
    assert!(body.contains(r#""title":"iPhone 9""#));
    assert!(body.contains(r#""price":549"#));
    assert!(body.contains(r#""stock":94"#));
    assert!(body.contains(r#""rating":4.69"#));
    assert!(body.contains(r#""images":["https://i.dummyjson.com/data/products/1/1.jpg","#));
    assert!(body.contains(r#""thumbnail":"https://i.dummyjson.com/data/products/1/thumbnail.jpg""#));
    assert!(body.contains(r#""description":"An apple mobile which is nothing like apple""#));
    assert!(body.contains(r#""brand":"Apple""#));
    assert!(body.contains(r#""category":"smartphones""#));
}

#[tokio::test]
async fn test_delete_request() {
    let url = "https://dummyjson.com/products/1";
    let response = TinyFetch::delete(url).await;

    assert!(response.is_ok());
    let body = response.unwrap();
    assert!(body.contains(r#""id":1"#));
    assert!(body.contains(r#""title":"iPhone 9""#));
    assert!(body.contains(r#""description":"An apple mobile which is nothing like apple""#));
    assert!(body.contains(r#""price":549"#));
    assert!(body.contains(r#""discountPercentage":12.96"#));
    assert!(body.contains(r#""rating":4.69"#));
    assert!(body.contains(r#""stock":94"#));
    assert!(body.contains(r#""brand":"Apple""#));
    assert!(body.contains(r#""category":"smartphones""#));
    assert!(body.contains(r#""thumbnail":"https://i.dummyjson.com/data/products/1/thumbnail.jpg""#));
    assert!(body.contains(r#""images":["https://i.dummyjson.com/data/products/1/1.jpg","#));
    assert!(body.contains(r#""isDeleted":true"#));
    assert!(body.contains(r#""deletedOn":"2023-05-21"#));
}
