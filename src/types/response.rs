use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[allow(unused)]
pub enum CommonResponse {
    Success,
    InvalidRequest,
}

#[allow(unused)]
pub enum RegisterResponse {
    Success,
    InvalidRequest,
    AccountExists,
    EmailOccupied,
    InvalidUsername,
    InvalidPassword,
    InvalidEmail,
    PasswordIsTooShort,
    UsernameIsTooShort,
    PasswordMismatch,
    EmailMismatch,
}

#[allow(unused)]
pub enum LoginResponse {
    InvalidRequest,
    WrongCredentials,
    AlreadyLinkedToDifferentAccount,
    PasswordIsTooShort,
    UsernameIsTooShort,
    AccountIsBanned,
    AccountIsNotActivated,
}

#[allow(unused)]
pub enum BackupResponse {
    InvalidRequest,
    WrongCredentials,
    BadLoginInfo,
    TooLarge,
    SomethingWentWrong,
}

#[allow(unused)]
pub enum LevelUploadResponse {
    Success,
    UploadingDisabled,
    TooFast,
    FailedToWriteLevel,
}

#[allow(unused)]
pub enum CommentsResponse {
    NothingFound,
}

impl IntoResponse for CommonResponse {
    fn into_response(self) -> Response {
        let body = match self {
            CommonResponse::Success => "1",
            CommonResponse::InvalidRequest => "-1",
        };

        (StatusCode::OK, body).into_response()
    }
}

impl IntoResponse for RegisterResponse {
    fn into_response(self) -> Response {
        let body = match self {
            RegisterResponse::Success => "1",
            RegisterResponse::InvalidRequest => "-1",
            RegisterResponse::AccountExists => "-2",
            RegisterResponse::EmailOccupied => "-3",
            RegisterResponse::InvalidUsername => "-4",
            RegisterResponse::InvalidPassword => "-5",
            RegisterResponse::InvalidEmail => "-6",
            RegisterResponse::PasswordMismatch => "-7",
            RegisterResponse::PasswordIsTooShort => "-8",
            RegisterResponse::UsernameIsTooShort => "-9",
            RegisterResponse::EmailMismatch => "-99",
        };

        (StatusCode::OK, body).into_response()
    }
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> Response {
        let body = match self {
            LoginResponse::InvalidRequest => "-1",
            LoginResponse::WrongCredentials => "-11",
            LoginResponse::AlreadyLinkedToDifferentAccount => "-10",
            LoginResponse::PasswordIsTooShort => "-8",
            LoginResponse::UsernameIsTooShort => "-9",
            LoginResponse::AccountIsBanned => "-12",
            LoginResponse::AccountIsNotActivated => "-13",
        };

        (StatusCode::OK, body).into_response()
    }
}

impl IntoResponse for BackupResponse {
    fn into_response(self) -> Response {
        let body = match self {
            BackupResponse::InvalidRequest => "-1",
            BackupResponse::WrongCredentials => "-2",
            BackupResponse::BadLoginInfo => "-3",
            BackupResponse::TooLarge => "-4",
            BackupResponse::SomethingWentWrong => "-5",
        };

        (StatusCode::OK, body).into_response()
    }
}

impl IntoResponse for LevelUploadResponse {
    fn into_response(self) -> Response {
        let body = match self {
            LevelUploadResponse::Success => "1",
            LevelUploadResponse::UploadingDisabled => "-1",
            LevelUploadResponse::TooFast => "-2",
            LevelUploadResponse::FailedToWriteLevel => "-3",
        };

        (StatusCode::OK, body).into_response()
    }
}

impl IntoResponse for CommentsResponse {
    fn into_response(self) -> Response {
        let body = match self {
            CommentsResponse::NothingFound => "-1",
        };

        (StatusCode::OK, body).into_response()
    }
}
