use std::env;

use rand::{distributions::Alphanumeric, Rng};
use sha2::{
    digest::{DynDigest, Update},
    Digest, Sha256,
};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio_postgres::{Client, GenericClient, NoTls};

use super::{Service, ServiceHandle, ServiceRequest};

pub type DBServiceHandle =
    ServiceHandle<DBServiceRequest, Result<DBServiceResponse, DBServiceError>>;
type DBServiceChannel = ServiceRequest<DBServiceRequest, Result<DBServiceResponse, DBServiceError>>;

pub struct DBService {
    sender: Sender<DBServiceChannel>,
    receiver: Receiver<DBServiceChannel>,
    client: Client,
}

pub enum DBServiceRequest {
    CreateUserDefault {
        username: String,
        password_hash: String,
    },
    CreatePasswordChallenge {
        username: String,
        challenge: [char; 64],
    },
    ConsumePasswordWithChallenge {
        username: String,
    },
    CreateUserGoogle {
        username: String,
        google_id: String,
    },
    CreateAccessTokenUsername {
        username: String,
    },
    CreateAccessTokenGoogle {
        google_id: String,
    },
}

pub enum DBServiceError {
    UnregisterdAccount,
    UserAlreadyExists,
    GoogleTaken,
    AuthenticationMismatch,
}

pub enum DBServiceResponse {
    Empty,
    AccessToken([char; 128]),
    PasswordHashWithChallenge([char; 64]),
}

impl DBService {
    pub async fn new() -> Self {
        let (sender, receiver) = channel(16);
        let (client, connection) = tokio_postgres::connect(
            &format!(
                "host={} user={} password={}",
                env::var("DB_IP").expect("No db ip provided"),
                env::var("DB_USERNAME").expect("No db username is provided"),
                env::var("DB_PASSWORD").expect("No db password is provided")
            ),
            NoTls,
        )
        .await
        .expect("Failed to connect to the databases");
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("db connection error: {}", e);
            }
        });
        Self {
            sender,
            receiver,
            client,
        }
    }

    async fn create_access_token_by_google_id(
        &mut self,
        google_id: String,
    ) -> Result<DBServiceResponse, DBServiceError> {
        let google_user_exists = self
            .client
            .query_opt(
                "SELECT 1 FROM google_id_users WHERE google_id = $1::TEXT",
                &[&google_id],
            )
            .await
            .unwrap();

        if google_user_exists.is_none() {
            return Err(DBServiceError::UnregisterdAccount);
        }

        let token: String = {
            let mut rng = rand::thread_rng();
            (0..128)
                .map(|_| rng.sample(Alphanumeric))
                .map(char::from)
                .collect()
        };

        self.client
            .query(
                "
                WITH google_user AS (
                    SELECT user_id
                    FROM google_id_users
                    WHERE google_id = $1::TEXT 
                )
                INSERT INTO access_token (token_id, user_id)
                SELECT $2::TEXT, user_id
                FROM google_user;
            ",
                &[&google_id, &token],
            )
            .await
            .ok();
        Ok(DBServiceResponse::AccessToken(
            token.chars().collect::<Vec<char>>().try_into().unwrap(),
        ))
    }

    async fn create_access_token_username(
        &mut self,
        username: String,
    ) -> Result<DBServiceResponse, DBServiceError> {
        let token: String = {
            let mut rng = rand::thread_rng();
            (0..128)
                .map(|_| rng.sample(Alphanumeric))
                .map(char::from)
                .collect()
        };
        self.client
            .query(
                "
            WITH users_id AS (
                SELECT user_id 
                FROM users 
                WHERE username = $1::TEXT
            ) 
            INSERT INTO access_token (token_id, user_id) 
            SELECT $2::TEXT, user_id 
            FROM users_id",
                &[&username, &token],
            )
            .await
            .unwrap();
        Ok(DBServiceResponse::AccessToken(
            token.chars().collect::<Vec<char>>().try_into().unwrap(),
        ))
    }

    async fn consume_password_hash_with_challenge(
        &mut self,
        username: String,
    ) -> Result<DBServiceResponse, DBServiceError> {
        let user_row = self
            .client
            .query(
                "SELECT password_hash, password_challenge FROM users WHERE username = $1::TEXT",
                &[&username],
            )
            .await
            .unwrap();
        if let Some(user) = user_row.get(0) {
            match (
                user.get::<_, Option<String>>("password_hash"),
                user.get::<_, Option<String>>("password_challenge"),
            ) {
                (Some(password_hash), Some(password_challenge)) => {
                    let mut sha = Sha256::new();
                    Digest::update(&mut sha, password_hash);
                    Digest::update(&mut sha, password_challenge);
                    let result: [char; 64] = format!("{:x}", sha.finalize())
                        .chars()
                        .collect::<Vec<_>>() // Collect into a Vec<char> temporarily
                        .try_into() // Convert Vec<char> to [char; 64]
                        .expect("Hash must be exactly 64 hex characters");
                    self.client
                        .query(
                            "UPDATE users
                            SET password_challenge = NULL 
                            WHERE username = $1::TEXT",
                            &[&username],
                        )
                        .await
                        .unwrap();
                    return Ok(DBServiceResponse::PasswordHashWithChallenge(result));
                }
                _ => return Err(DBServiceError::AuthenticationMismatch),
            }
        }
        return Err(DBServiceError::UnregisterdAccount);
    }

    async fn create_user_google(
        &mut self,
        username: String,
        google_id: String,
    ) -> Result<DBServiceResponse, DBServiceError> {
        let user_exists = self
            .client
            .query_opt(
                "SELECT 1 FROM users WHERE username = $1::TEXT",
                &[&username],
            )
            .await
            .unwrap();

        if user_exists.is_some() {
            return Err(DBServiceError::UserAlreadyExists);
        }

        let google_exists = self
            .client
            .query_opt(
                "SELECT 1 FROM google_id_users WHERE google_id = $1::TEXT",
                &[&google_id],
            )
            .await
            .unwrap();

        if google_exists.is_some() {
            return Err(DBServiceError::GoogleTaken);
        }

        self.client
            .query(
                "
                WITH inserted_user AS (
                    INSERT INTO users (username)
                    VALUES ($1::TEXT)
                    RETURNING user_id
                )
                INSERT INTO google_id_users (google_id, user_id)
                VALUES ($2::TEXT, (SELECT user_id FROM inserted_user));
            ",
                &[&username, &google_id],
            )
            .await
            .unwrap();
        Ok(DBServiceResponse::Empty)
    }

    async fn create_password_challenge(
        &mut self,
        username: String,
        challenge: [char; 64],
    ) -> Result<DBServiceResponse, DBServiceError> {
        self.client
            .query(
                "UPDATE users
SET password_challenge = $1::TEXT 
WHERE username = $2::TEXT",
                &[&challenge.iter().collect::<String>(), &username],
            )
            .await
            .unwrap();
        return Ok(DBServiceResponse::Empty);
    }

    async fn create_user_default(
        &mut self,
        username: String,
        password_hash: String,
    ) -> Result<DBServiceResponse, DBServiceError> {
        let user_exists = self
            .client
            .query_opt(
                "SELECT 1 FROM users WHERE username = $1::TEXT",
                &[&username],
            )
            .await
            .unwrap();

        if user_exists.is_some() {
            return Err(DBServiceError::UserAlreadyExists);
        }

        self.client
            .query(
                "
                INSERT INTO users (username, password_hash)
                VALUES ($1::TEXT, $2::TEXT)
            ",
                &[&username, &password_hash],
            )
            .await
            .ok();
        Ok(DBServiceResponse::Empty)
    }
}

impl Service<DBServiceRequest, Result<DBServiceResponse, DBServiceError>> for DBService {
    fn get_sender(&self) -> Sender<DBServiceChannel> {
        self.sender.clone()
    }

    fn get_receiver(&mut self) -> &mut Receiver<DBServiceChannel> {
        &mut self.receiver
    }

    async fn process(
        &mut self,
        data: DBServiceRequest,
    ) -> Result<DBServiceResponse, DBServiceError> {
        match data {
            DBServiceRequest::CreateUserDefault {
                username,
                password_hash,
            } => self.create_user_default(username, password_hash).await,
            DBServiceRequest::CreateUserGoogle {
                username,
                google_id,
            } => self.create_user_google(username, google_id).await,
            DBServiceRequest::CreateAccessTokenGoogle { google_id } => {
                self.create_access_token_by_google_id(google_id).await
            }
            DBServiceRequest::CreatePasswordChallenge {
                username,
                challenge,
            } => self.create_password_challenge(username, challenge).await,
            DBServiceRequest::ConsumePasswordWithChallenge { username } => {
                self.consume_password_hash_with_challenge(username).await
            }
            DBServiceRequest::CreateAccessTokenUsername { username } => {
                self.create_access_token_username(username).await
            }
        }
    }
}
