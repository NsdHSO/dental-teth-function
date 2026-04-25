use actix_web::HttpResponse;
use serde::Serialize;

use crate::{error_handler::CustomError, http_response_builder};

/// A generic helper function to convert a `Result` into a standardized `HttpResponse` or pass through an error.
///
/// This function is designed for use in `actix-web` handlers to reduce boilerplate code for success and
/// error responses. It takes a `Result` from a service or business logic layer. If the result is `Ok`,
/// it constructs an `HttpResponse` with a 200 OK status and a standardized JSON body. If the result is `Err`,
/// it simply returns the `CustomError`, allowing Actix to handle it.
///
/// # Arguments
///
/// * `response_result`: A `Result<T, CustomError>` where `T` is the data payload to be serialized.
///   - The `Ok` variant contains the data (`T`) that will be wrapped in the JSON response.
///   - The `Err` variant contains a `CustomError` that is propagated.
///
/// # Type Parameters
///
/// * `T`: The type of the successful response payload. This type must implement the
///   `serde::Serialize` trait to be converted into JSON.
///
/// # Returns
///
/// A `Result<HttpResponse, CustomError>`
/// - `Ok(HttpResponse)`: An HTTP response with a 200 OK status code.
/// - `Err(CustomError)`: The original error from the input `Result`.
///
/// # Examples
///
/// ```no_run
/// use actix_web::HttpResponse;
/// use serde::Serialize;
/// use http_response::{CustomError, HttpCodeW, prepared_response::check_response_ok_or_return_error};
///
/// #[derive(Serialize)]
/// struct UserProfile {
///     id: i32,
///     username: String,
/// }
///
/// async fn get_user_handler() -> Result<HttpResponse, CustomError> {
///     let user_result: Result<UserProfile, CustomError> = Ok(UserProfile {
///         id: 1,
///         username: "test_user".to_string()
///     });
///     
///     check_response_ok_or_return_error(user_result)
/// }
/// ```
pub fn check_response_ok_or_return_error<T: Serialize>(
    response_result: Result<T, CustomError>,
) -> Result<HttpResponse, CustomError> {
    match response_result {
        Ok(response) => {
            let response = http_response_builder::ok(response);
            Ok(HttpResponse::Ok().json(response))
        }
        Err(err) => Err(err),
    }
}
