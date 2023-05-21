TinyFetch is a lightweight Rust library for making HTTP requests with ease. It provides a simple and coherent interface for performing `GET`, `POST`, `PUT`, `PATCH`, and `DELETE` requests. It is built on top of the reqwest crate and designed to be easy to use and maintain.

## Usage

Add the `tinyfetch` and `tokio` crates as dependencies in your Rust project's `Cargo.toml` file:

```toml
[dependencies]
tinyfetch = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

Import the `TinyFetch` struct into your Rust code:

```rust
use tinyfetch::TinyFetch;
```

Now you can use the TinyFetch methods to perform HTTP requests. Here's an example that demonstrates how to use each method:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GET
    let response = TinyFetch::get("https://dummyjson.com/products/").await?;
    println!("GET Response: {}", response);

    // POST
    let body = r#"
    {
        "name": "Example Product",
        "price": 9.99,
        "quantity": 10
    }
    "#;

    let response = TinyFetch::post("https://dummyjson.com/products/add", body).await?;
    println!("POST Response: {}", response);

    // PUT
    let body = r#"
        {
            "name": "Updated Product",
            "price": 19.99,
            "quantity": 5
        }
    "#;

    let response = TinyFetch::put("https://dummyjson.com/products/1", body).await?;
    println!("PUT Response: {}", response);

    // PATCH
    let body = r#"
        {
            "price": 14.99
        }
    "#;

    let response = TinyFetch::patch("https://dummyjson.com/products/1", body).await?;
    println!("PATCH Response: {}", response);

    // DELETE
    let response = TinyFetch::delete("https://dummyjson.com/products/1").await?;
    println!("DELETE Response: {}", response);

    Ok(())
}
```

Make sure to use the `#[tokio::main]` attribute on your `main` function to enable asynchronous execution with Tokio.

## Supported HTTP Methods

TinyFetch supports the following HTTP methods:

- GET: `TinyFetch::get(url: &str) -> Result<String, Box<dyn std::error::Error>>`
- POST: `TinyFetch::post(url: &str, body: &str) -> Result<String, Box<dyn std::error::Error>>`
- PUT: `TinyFetch::put(url: &str, body: &str) -> Result<String, Box<dyn std::error::Error>>`
- PATCH: `TinyFetch::patch(url: &str, body: &str) -> Result<String, Box<dyn std::error::Error>>`
- DELETE: `TinyFetch::delete(url: &str) -> Result<String, Box<dyn std::error::Error>>`

Each method takes the URL as a string parameter and returns a Result containing the response body as a string or an error.

## Error Handling

TinyFetch uses Rust's error handling mechanism to propagate errors. If an error occurs during the HTTP request, it will be returned as a Box<dyn std::error::Error> . You can use standard Rust error handling techniques, such as `?` or pattern matching, to handle these errors in your code.

## Run tests

```bash
cargo test
```

## Contributing

Contributions to TinyFetch are welcome! If you encounter any issues or have suggestions for improvements, please open an issue on the GitHub repository. Pull requests are also appreciated.

To contribute to the project, follow these steps:

1. Fork the repository and clone it to your local machine.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the code passes all tests.
4. Write tests for any new functionality or changes.
5. Commit your changes and push them to your forked repository.
6. Submit a pull request on the main repository.

Please ensure that your code follows the established coding style and conventions. Add tests to cover new functionality and ensure that existing tests pass.

## License

TinyFetch is open-source software licensed under the MIT License. See the LICENSE file for more details.

## Contact

For any further inquiries or questions, feel free to contact the maintainers of TinyFetch via the GitHub repository.

## Acknowledgments

TinyFetch is inspired by the simplicity and functionality of Ruby's [HTTParty library](https://github.com/jnunemaker/httparty). Special thanks to the developers of [reqwest](https://github.com/seanmonstar/reqwest) and [tokio](https://github.com/tokio-rs/tokio) for providing the necessary tools and libraries to make this project possible.
