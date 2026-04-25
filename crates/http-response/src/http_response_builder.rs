use crate::{create_response, response_object::ResponseObject, HttpCodeW};

/// Creates a successful HTTP response with status code 200 OK.
///
/// This function wraps the provided message in a ResponseObject with HTTP 200 status,
/// indicating that the request has succeeded. The ResponseObject can be serialized
/// to JSON containing both the message and HTTP status code.
///
/// # Arguments
///
/// * `message`: The response payload of type T to be returned to the client (must implement Serialize for JSON serialization)
///
/// returns: ResponseObject<T> - A response object containing the message and OK status (HttpCodeW::OK)
///
/// # Examples
///
/// ```
/// # use http_response::ResponseObject;
/// # use http_response::{create_response, HttpCodeW};
/// # fn ok<T>(message: T) -> ResponseObject<T> { create_response(message, HttpCodeW::OK) }
/// let response = ok("Request processed successfully");
/// // Serializes to: {"message": "Request processed successfully", "code": 200}
/// ```
#[allow(dead_code)]
pub fn ok<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::OK)
}

/// Creates an HTTP response with status code 201 Created.
///
/// This function is typically used when a new resource has been successfully created
/// on the server as a result of the request. Uses the `create_response` helper internally.
///
/// # Arguments
///
/// * `message`: The response payload of type T, usually containing the created resource (must implement Serialize for JSON serialization)
///
/// returns: ResponseObject<T> - A response object containing the message and Created status (HttpCodeW::Created)
///
/// # Examples
///
/// ```
/// # use http_response::ResponseObject;
/// # use http_response::{create_response, HttpCodeW};
/// # fn created<T>(message: T) -> ResponseObject<T> { create_response(message, HttpCodeW::Created) }
/// let response = created("User created successfully");
/// // Serializes to: {"message": "User created successfully", "code": 201}
/// ```
#[allow(dead_code)]
pub fn created<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::Created)
}

/// Creates an HTTP response with status code 204 No Content.
///
/// This function indicates that the server has successfully processed the request
/// but is not returning any content. Commonly used for DELETE operations.
/// Uses the `create_response` helper internally.
///
/// # Arguments
///
/// * `message`: The response payload of type T (often empty or confirmation message, must implement Serialize for JSON serialization)
///
/// returns: ResponseObject<T> - A response object containing the message and NoContent status (HttpCodeW::NoContent)
///
/// # Examples
///
/// ```
/// # use http_response::ResponseObject;
/// # use http_response::{create_response, HttpCodeW};
/// # fn no_content<T>(message: T) -> ResponseObject<T> { create_response(message, HttpCodeW::NoContent) }
/// let response = no_content("Resource deleted successfully");
/// // Serializes to: {"message": "Resource deleted successfully", "code": 204}
/// ```
#[allow(dead_code)]
pub fn no_content<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::NoContent)
}

/// Creates an HTTP response with status code 400 Bad Request.
///
/// This function indicates that the server cannot process the request due to
/// client error, such as malformed syntax or invalid request parameters.
/// Uses the `create_response` helper internally.
///
/// # Arguments
///
/// * `message`: The error message or details of type T explaining why the request is invalid (must implement Serialize for JSON serialization)
///
/// returns: ResponseObject<T> - A response object containing the error message and BadRequest status (HttpCodeW::BadRequest)
///
/// # Examples
///
/// ```
/// # use http_response::ResponseObject;
/// # use http_response::{create_response, HttpCodeW};
/// # fn bad_request<T>(message: T) -> ResponseObject<T> { create_response(message, HttpCodeW::BadRequest) }
/// let response = bad_request("Missing required field: email");
/// // Serializes to: {"message": "Missing required field: email", "code": 400}
/// ```
#[allow(dead_code)]
pub fn bad_request<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::BadRequest)
}

/// Creates an HTTP response with status code 401 Unauthorized.
///
/// This function indicates that the request requires authentication or
/// the provided authentication credentials are invalid.
/// Uses the `create_response` helper internally.
///
/// # Arguments
///
/// * `message`: The error message of type T explaining the authentication failure (must implement Serialize for JSON serialization)
///
/// returns: ResponseObject<T> - A response object containing the error message and Unauthorized status (HttpCodeW::Unauthorized)
///
/// # Examples
///
/// ```
/// # use http_response::{create_response, HttpCodeW, ResponseObject};
/// # fn unauthorized<T>(message: T) -> ResponseObject<T> { create_response(message, HttpCodeW::Unauthorized) }
/// let response = unauthorized("Invalid authentication token");
/// // Serializes to: {"message": "Invalid authentication token", "code": 401}
/// ```
#[allow(dead_code)]
pub fn unauthorized<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::Unauthorized)
}

/// Creates an HTTP response with status code 409 Conflict.
///
/// This function indicates that the request conflicts with the current state
/// of the server, such as attempting to create a resource that already exists.
/// Uses the `create_response` helper internally.
///
/// # Arguments
///
/// * `message`: The error message of type T describing the conflict (must implement Serialize for JSON serialization)
///
/// returns: ResponseObject<T> - A response object containing the error message and Conflict status (HttpCodeW::Conflict)
///
/// # Examples
///
/// ```
/// # use http_response::ResponseObject;
/// # use http_response::{create_response, HttpCodeW};
/// # fn conflict<T>(message: T) -> ResponseObject<T> { create_response(message, HttpCodeW::Conflict) }
/// let response = conflict("User with this email already exists");
/// // Serializes to: {"message": "User with this email already exists", "code": 409}
/// ```
#[allow(dead_code)]
pub fn conflict<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::Conflict)
}

/// Creates an HTTP response with status code 404 Not Found.
///
/// This function indicates that the server cannot find the requested resource.
/// The URL is not recognized or the resource does not exist.
/// Uses the `create_response` helper internally.
///
/// # Arguments
///
/// * `message`: The error message of type T indicating what resource was not found (must implement Serialize for JSON serialization)
///
/// returns: ResponseObject<T> - A response object containing the error message and NotFound status (HttpCodeW::NotFound)
///
/// # Examples
///
/// ```
/// # use http_response::ResponseObject;
/// # use http_response::{create_response, HttpCodeW};
/// # fn not_found<T>(message: T) -> ResponseObject<T> { create_response(message, HttpCodeW::NotFound) }
/// let response = not_found("User not found");
/// // Serializes to: {"message": "User not found", "code": 404}
/// ```
#[allow(dead_code)]
pub fn not_found<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::NotFound)
}

/// Creates an HTTP response with status code 500 Internal Server Error.
///
/// This function indicates that the server encountered an unexpected condition
/// that prevented it from fulfilling the request.
/// Uses the `create_response` helper internally.
///
/// # Arguments
///
/// * `message`: The error message of type T describing the server error (must implement Serialize for JSON serialization)
///
/// returns: ResponseObject<T> - A response object containing the error message and InternalServerError status (HttpCodeW::InternalServerError)
///
/// # Examples
///
/// ```
/// # use http_response::ResponseObject;
/// # use http_response::{create_response, HttpCodeW};
/// # fn internal_server_error<T>(message: T) -> ResponseObject<T> { create_response(message, HttpCodeW::InternalServerError) }
/// let response = internal_server_error("Database connection failed");
/// // Serializes to: {"message": "Database connection failed", "code": 500}
/// ```
#[allow(dead_code)]
pub fn internal_server_error<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::InternalServerError)
}

/// Creates an HTTP response with status code 501 Not Implemented.
///
/// This function indicates that the server does not support the functionality
/// required to fulfill the request. This is typically used for features that
/// are planned but not yet implemented.
/// Uses the `create_response` helper internally.
///
/// # Arguments
///
/// * `message`: The message of type T indicating what functionality is not implemented (must implement Serialize for JSON serialization)
///
/// returns: ResponseObject<T> - A response object containing the message and NotImplemented status (HttpCodeW::NotImplemented)
///
/// # Examples
///
/// ```
/// # use http_response::ResponseObject;
/// # use http_response::{create_response, HttpCodeW};
/// # fn not_implemented<T>(message: T) -> ResponseObject<T> { create_response(message, HttpCodeW::NotImplemented) }
/// let response = not_implemented("File upload feature coming soon");
/// // Serializes to: {"message": "File upload feature coming soon", "code": 501}
/// ```
#[allow(dead_code)]
pub fn not_implemented<T>(message: T) -> ResponseObject<T> {
    create_response(message, HttpCodeW::NotImplemented)
}
