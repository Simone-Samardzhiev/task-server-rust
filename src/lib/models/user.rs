use crate::utils::api_error_response::APIErrorResponse;
use axum::http::StatusCode;
use serde::Deserialize;

/// `UserPayload` holds used data from requests.
#[derive(Deserialize)]
pub struct UserPayload {
    pub email: String,
    pub username: String,
    pub password: String,
}

impl UserPayload {
    pub fn validate(&self) -> Option<APIErrorResponse> {
        if let Some(error) = self.validate_email() {
            return Some(error);
        }

        if let Some(error) = self.validate_username() {
            return Some(error);
        }

        if let Some(error) = self.validate_password() {
            return Some(error);
        }

        None
    }

    fn validate_email(&self) -> Option<APIErrorResponse> {
        let parts: Vec<&str> = self.email.split("@").collect();
        if parts.len() != 2 {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Invalid email format"),
            ));
        }

        if parts[0].is_empty() || parts[1].is_empty() {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Invalid email format(missing local part or domain)"),
            ));
        }

        let domain_parts: Vec<&str> = parts[1].split(".").collect();
        if domain_parts.len() < 2 {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Invalid email format(invalid domain)"),
            ));
        }

        if domain_parts[0].len() < 2 || domain_parts[1].len() < 2 {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Invalid email format(invalid domain)"),
            ));
        }

        None
    }

    fn validate_username(&self) -> Option<APIErrorResponse> {
        if self.username.len() < 8 {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Username should be more than 7 characters"),
            ));
        }

        None
    }

    fn validate_password(&self) -> Option<APIErrorResponse> {
        if self.password.len() < 8 {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Password is too short"),
            ));
        }

        if !self.password.chars().any(|c| c.is_numeric()) {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Password must contain at least one number"),
            ));
        }

        if !self.password.chars().any(|c| c.is_uppercase()) {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Password must contain at least one uppercase letter"),
            ));
        }

        if !self.password.chars().any(|c| c.is_lowercase()) {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Password must contain at least one lowercase letter"),
            ));
        }

        if !self.password.chars().any(|c| c.is_ascii_punctuation()) {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Password must contain at least one special character"),
            ));
        }

        None
    }
}

/// `User` holds used data.
pub struct User {
    id: i64,
    email: String,
    username: String,
    password: String,
}

mod tests {
    use super::*;

    struct TestCase {
        user_payload: UserPayload,
        expected: Option<APIErrorResponse>,
    }

    impl TestCase {
        fn new(user_payload: UserPayload, expected: Option<APIErrorResponse>) -> Self {
            Self {
                user_payload,
                expected,
            }
        }
    }

    #[test]
    fn test_user_validate_email() {
        let tests: Vec<TestCase> = vec![
            TestCase::new(
                UserPayload {
                    email: String::from("missing"),
                    username: String::from("Valid_Username"),
                    password: String::from("Password_0123"),
                },
                Some(APIErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    String::from("Invalid email format"),
                )),
            ),
            TestCase::new(
                UserPayload {
                    email: String::from("@example.com"),
                    username: String::from("Valid_Username"),
                    password: String::from("Password_0123"),
                },
                Some(APIErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    String::from("Invalid email format(missing local part or domain)"),
                )),
            ),
            TestCase::new(
                UserPayload {
                    email: String::from("email@examplecom"),
                    username: String::from("Valid_Username"),
                    password: String::from("Password_0123"),
                },
                Some(APIErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    String::from("Invalid email format(invalid domain)"),
                )),
            ),
            TestCase::new(
                UserPayload {
                    email: String::from("email@example.c"),
                    username: String::from("Valid_Username"),
                    password: String::from("Password_0123"),
                },
                Some(APIErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    String::from("Invalid email format(invalid domain)"),
                )),
            ),
            TestCase::new(
                UserPayload {
                    email: String::from("email@e.com"),
                    username: String::from("Valid_Username"),
                    password: String::from("Password_0123"),
                },
                Some(APIErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    String::from("Invalid email format(invalid domain)"),
                )),
            ),
            TestCase::new(
                UserPayload {
                    email: String::from("email@example.com"),
                    username: String::from("Valid_Username"),
                    password: String::from("Password_0123"),
                },
                None,
            ),
        ];

        for test in tests {
            assert_eq!(test.expected, test.user_payload.validate());
        }
    }

    #[test]
    fn test_user_validate_username() {
        let tests = vec![TestCase::new(
            UserPayload {
                email: String::from("email@example.com"),
                username: String::from("user"),
                password: String::from("Password_0123"),
            },
            Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Username should be more than 7 characters"),
            )),
        )];

        for test in tests {
            assert_eq!(test.expected, test.user_payload.validate());
        }
    }

    #[test]
    fn test_user_validate_password() {
        let tests = vec![
            TestCase::new(
                UserPayload {
                    email: String::from("email@example.com"),
                    username: String::from("Valid_Username"),
                    password: String::from("pass"),
                },
                Some(APIErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    String::from("Password is too short"),
                )),
            ),
            TestCase::new(
                UserPayload {
                    email: String::from("email@example.com"),
                    username: String::from("Valid_Username"),
                    password: String::from("Password"),
                },
                Some(APIErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    String::from("Password must contain at least one number"),
                )),
            ),
            TestCase::new(
                UserPayload {
                    email: String::from("email@example.com"),
                    username: String::from("Valid_Username"),
                    password: String::from("password1"),
                },
                Some(APIErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    String::from("Password must contain at least one uppercase letter"),
                )),
            ),
            TestCase::new(
                UserPayload {
                    email: String::from("email@example.com"),
                    username: String::from("Valid_Username"),
                    password: String::from("PASSWORD1"),
                },
                Some(APIErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    String::from("Password must contain at least one lowercase letter"),
                )),
            ),
            TestCase::new(
                UserPayload {
                    email: String::from("email@example.com"),
                    username: String::from("Valid_Username"),
                    password: String::from("Password1"),
                },
                Some(APIErrorResponse::new(
                    StatusCode::BAD_REQUEST,
                    String::from("Password must contain at least one special character"),
                )),
            ),
        ];
    }
}
