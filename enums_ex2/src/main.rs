// Define an enum 'HTTPResponse' to represent different http status codes
#[derive(Debug)]
enum HTTPResponse {
    Ok,
    NotFound,
    InternalServerError,
}

impl HTTPResponse {
    // Methode to return a user-friendly message based on the response code
    fn show_message(&self) -> &str {
        match self {
            HTTPResponse::Ok => "200 Request successful",
            HTTPResponse::NotFound => "404 Page not found",
            HTTPResponse::InternalServerError => "500 Internal server error",
        }
    }
}

fn main() {
    let response = HTTPResponse::Ok;
    println!("Response message: {}", response.show_message());

    let response = HTTPResponse::NotFound;
    println!("Response message: {}", response.show_message());

    let response = HTTPResponse::InternalServerError;
    println!("Response message: {}", response.show_message());

}
