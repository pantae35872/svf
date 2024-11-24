use axum::Json;
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use reqwest::StatusCode;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::{utils::get_google_info, web_server::BackendResponse};

use super::{
    db_service::{DBServiceError, DBServiceHandle, DBServiceRequest, DBServiceResponse},
    Service, ServiceHandle, ServiceRequest,
};

pub type AuthenticationServiceHandle = ServiceHandle<
    AuthenticationServiceRequest,
    Result<AuthenticationServiceResponse, AuthenticationServiceError>,
>;

type AuthenticationServiceChannel = ServiceRequest<
    AuthenticationServiceRequest,
    Result<AuthenticationServiceResponse, AuthenticationServiceError>,
>;

pub struct AuthenticationService {
    sender: Sender<AuthenticationServiceChannel>,
    receiver: Receiver<AuthenticationServiceChannel>,
    db: DBServiceHandle,
    client: reqwest::Client,
}

pub enum AuthenticationServiceRequest {
    GoogleLogin {
        google_access_token: String,
    },
    DefaultLogin {
        username: String,
        password_hash: String,
    },
    DefaultSignup {
        username: String,
        password_hash: String,
    },
    GoogleSignup {
        google_access_token: String,
        username: String,
    },
    PasswordChallenge {
        username: String,
    },
}

#[derive(Debug, Clone)]
pub enum AuthenticationServiceError {
    InvalidUsernameRegex,
    InvalidUsernameLength,
    InvalidPassword,
    InvalidGoogleToken,
    InvalidAccessToken,
    AuthenticationMismatch,
    UnregisteredAccount,
    UsernameTaken,
    GoogleTaken,
    WrongPassword,
}

impl Into<Json<BackendResponse>> for AuthenticationServiceError {
    fn into(self) -> Json<BackendResponse> {
        Json(BackendResponse::Error(
            match self {
                Self::UnregisteredAccount => "Trying to login into unregistered account.",
                Self::InvalidUsernameRegex => "Username can only contain lowercase and uppercase English letters, as well as the characters '-' and '_'.",
                Self::InvalidUsernameLength => "Username length can only be in the range of 3 to 20 characters.",
                Self::InvalidGoogleToken => "Invalid Google Token",
                Self::InvalidPassword => "Invalid Password",
                Self::InvalidAccessToken => "invalid_access_token",
                Self::UsernameTaken => "The username has already been taken.",
                Self::GoogleTaken => "This Google account has already been registered in the system.",
                Self::AuthenticationMismatch => "This account has been registered with different authentication method.",
                Self::WrongPassword => "Incorrect password.",
            }
            .to_string(),
        ))
    }
}

impl Into<StatusCode> for AuthenticationServiceError {
    fn into(self) -> StatusCode {
        match self {
            Self::UnregisteredAccount
            | Self::InvalidPassword
            | Self::InvalidAccessToken
            | Self::InvalidGoogleToken => StatusCode::UNAUTHORIZED,
            Self::InvalidUsernameRegex
            | Self::UsernameTaken
            | Self::InvalidUsernameLength
            | Self::GoogleTaken
            | Self::AuthenticationMismatch
            | Self::WrongPassword => StatusCode::BAD_REQUEST,
        }
    }
}

impl From<DBServiceError> for AuthenticationServiceError {
    fn from(value: DBServiceError) -> Self {
        match value {
            DBServiceError::UnregisterdAccount => Self::UnregisteredAccount,
            DBServiceError::UserAlreadyExists => Self::UsernameTaken,
            DBServiceError::GoogleTaken => Self::GoogleTaken,
            DBServiceError::AuthenticationMismatch => Self::AuthenticationMismatch,
        }
    }
}

pub enum AuthenticationServiceResponse {
    AccessToken([char; 128]),
    PasswordChallenge([char; 64]),
    Empty,
}

impl AuthenticationService {
    pub fn new(db: DBServiceHandle) -> Self {
        let (sender, receiver) = channel(16);
        Self {
            sender,
            receiver,
            db,
            client: reqwest::Client::new(),
        }
    }

    pub fn verify_username(username: &str) -> Result<(), AuthenticationServiceError> {
        let regex = Regex::new(r"^[a-zA-Z_-]+$").unwrap();
        if !regex.is_match(&username) {
            return Err(AuthenticationServiceError::InvalidUsernameRegex);
        }
        if username.len() < 3 || username.len() > 20 {
            return Err(AuthenticationServiceError::InvalidUsernameLength);
        }
        Ok(())
    }

    pub async fn signup_google(
        &mut self,
        username: String,
        google_access_token: String,
    ) -> Result<AuthenticationServiceResponse, AuthenticationServiceError> {
        Self::verify_username(&username)?;
        self.db
            .request(DBServiceRequest::CreateUserGoogle {
                username,
                google_id: get_google_info(&self.client, google_access_token)
                    .await
                    .ok_or(AuthenticationServiceError::InvalidGoogleToken)?
                    .id,
            })
            .await?;
        Ok(AuthenticationServiceResponse::Empty)
    }

    pub async fn signup_default(
        &mut self,
        username: String,
        password_hash: String,
    ) -> Result<AuthenticationServiceResponse, AuthenticationServiceError> {
        Self::verify_username(&username)?;
        self.db
            .request(DBServiceRequest::CreateUserDefault {
                username,
                password_hash,
            })
            .await?;
        Ok(AuthenticationServiceResponse::Empty)
    }

    pub async fn username_login(
        &mut self,
        username: String,
        password_hash: String,
    ) -> Result<AuthenticationServiceResponse, AuthenticationServiceError> {
        let hash = match self
            .db
            .request(DBServiceRequest::ConsumePasswordWithChallenge {
                username: username.clone(),
            })
            .await?
        {
            DBServiceResponse::PasswordHashWithChallenge(hash) => hash,
            _ => unreachable!(),
        };
        if hash
            == TryInto::<[char; 64]>::try_into(password_hash.chars().collect::<Vec<char>>())
                .unwrap()
        {
            let token = match self
                .db
                .request(DBServiceRequest::CreateAccessTokenUsername { username })
                .await?
            {
                DBServiceResponse::AccessToken(token) => token,
                _ => unreachable!(),
            };
            return Ok(AuthenticationServiceResponse::AccessToken(token));
        }
        return Err(AuthenticationServiceError::WrongPassword);
    }

    pub async fn password_challenge(
        &mut self,
        username: String,
    ) -> Result<AuthenticationServiceResponse, AuthenticationServiceError> {
        let challenge = TryInto::<[char; 64]>::try_into({
            let mut rng = rand::thread_rng();
            (0..64)
                .map(|_| rng.sample(Alphanumeric))
                .map(char::from)
                .collect::<Vec<char>>()
        })
        .unwrap();
        self.db
            .request(DBServiceRequest::CreatePasswordChallenge {
                username,
                challenge,
            })
            .await?;
        return Ok(AuthenticationServiceResponse::PasswordChallenge(challenge));
    }

    pub async fn google_login(
        &mut self,
        access_token: String,
    ) -> Result<AuthenticationServiceResponse, AuthenticationServiceError> {
        match self
            .db
            .request(DBServiceRequest::CreateAccessTokenGoogle {
                google_id: get_google_info(&self.client, access_token)
                    .await
                    .ok_or(AuthenticationServiceError::InvalidGoogleToken)?
                    .id,
            })
            .await?
        {
            DBServiceResponse::AccessToken(token) => {
                Ok(AuthenticationServiceResponse::AccessToken(token))
            }
            _ => unreachable!(),
        }
    }
}

impl
    Service<
        AuthenticationServiceRequest,
        Result<AuthenticationServiceResponse, AuthenticationServiceError>,
    > for AuthenticationService
{
    fn get_sender(&self) -> Sender<AuthenticationServiceChannel> {
        self.sender.clone()
    }

    fn get_receiver(&mut self) -> &mut Receiver<AuthenticationServiceChannel> {
        &mut self.receiver
    }

    async fn process(
        &mut self,
        data: AuthenticationServiceRequest,
    ) -> Result<AuthenticationServiceResponse, AuthenticationServiceError> {
        match data {
            AuthenticationServiceRequest::GoogleLogin {
                google_access_token,
            } => self.google_login(google_access_token).await,
            AuthenticationServiceRequest::DefaultSignup {
                username,
                password_hash,
            } => self.signup_default(username, password_hash).await,
            AuthenticationServiceRequest::GoogleSignup {
                google_access_token,
                username,
            } => self.signup_google(username, google_access_token).await,
            AuthenticationServiceRequest::PasswordChallenge { username } => {
                self.password_challenge(username).await
            }
            AuthenticationServiceRequest::DefaultLogin {
                username,
                password_hash,
            } => self.username_login(username, password_hash).await,
        }
    }
}
