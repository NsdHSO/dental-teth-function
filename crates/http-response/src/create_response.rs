use crate::{response_object::ResponseObject, HttpCodeW};

/// Creates a ResponseObject with the provided message and HTTP status code.
///
/// This is the core helper function that all other response functions use internally.
/// It constructs a ResponseObject struct containing the message and HTTP status code,
/// which can be serialized to JSON with both fields.
///
/// # Arguments
///
/// * `message`: The response payload of type T (must implement Serialize for JSON serialization)
/// * `code`: The HTTP status code as HttpCodeW enum variant
///
/// returns: ResponseObject<T> - A response object with the provided message and status code
///
/// # Examples
///
/// ```
/// use http_response::{create_response, HttpCodeW};
///
/// let response = create_response("Custom message", HttpCodeW::OK);
/// // Serializes to: {"message": "Custom message", "code": 200}
/// ```
pub fn create_response<T>(message: T, code: HttpCodeW) -> ResponseObject<T> {
    ResponseObject { message, code }
}
