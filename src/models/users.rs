use async_trait::async_trait;
use chrono::offset::Utc;
use loco_rs::{auth::jwt, hash, prelude::*};
use serde::{Deserialize, Serialize};
use sqlx::{
    types::{chrono::NaiveDateTime, Uuid},
    PgPool,
};

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Model {
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub api_key: Uuid,
    pub username: String,
    pub reset_token: Option<Uuid>,
    pub reset_sent_at: Option<NaiveDateTime>,
    pub email_verification_token: Option<Uuid>,
    pub email_verification_sent_at: Option<NaiveDateTime>,
    pub email_verified_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterParams {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct Validator {
    #[validate(length(min = 2, message = "Name must be at least 2 characters long."))]
    pub username: String,
    #[validate(email)]
    pub email: String,
}

impl Validatable for Model {
    fn validator(&self) -> Box<dyn Validate> {
        Box::new(Validator {
            username: self.username.clone(),
            email: self.email.clone(),
        })
    }
}

#[async_trait]
impl Authenticable for Model {
    async fn find_by_api_key(db: &PgPool, api_key: Uuid) -> ModelResult<Self> {
        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE api_key = $1", api_key)
            .fetch_optional(db)
            .await?;
        user.ok_or(ModelError::EntityNotFound)
    }

    async fn find_by_claims_key(db: &PgPool, claims_key: Uuid) -> ModelResult<Self> {
        Self::find_by_id(db, claims_key).await
    }
}

impl Model {
    /// finds a user by the provided email
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_email(db: &PgPool, email: &str) -> ModelResult<Self> {
        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(db)
            .await?;

        user.ok_or(ModelError::EntityNotFound)
    }

    /// finds a user by the provided verification token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_verification_token(db: &PgPool, token: Uuid) -> ModelResult<Self> {
        let user = sqlx::query_as!(
            Self,
            "SELECT * FROM users WHERE email_verification_token = $1",
            token
        )
        .fetch_optional(db)
        .await?;

        user.ok_or(ModelError::EntityNotFound)
    }

    /// /// finds a user by the provided reset token
    ///
    /// # Errors
    ///
    /// When could not find user by the given token or DB query error
    pub async fn find_by_reset_token(db: &PgPool, token: Uuid) -> ModelResult<Self> {
        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE reset_token = $1", token)
            .fetch_optional(db)
            .await?;

        user.ok_or(ModelError::EntityNotFound)
    }

    /// finds a user by the provided id
    ///
    /// # Errors
    ///
    /// When could not find user  or DB query error
    pub async fn find_by_id(db: &PgPool, id: Uuid) -> ModelResult<Self> {
        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(db)
            .await?;

        user.ok_or(ModelError::EntityNotFound)
    }

    /// finds a user by the provided username
    ///
    /// # Errors
    ///
    /// When could not find user  or DB query error
    pub async fn find_by_username(db: &PgPool, username: &str) -> ModelResult<Self> {
        let user = sqlx::query_as!(Self, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(db)
            .await?;

        user.ok_or(ModelError::EntityNotFound)
    }

    /// Verifies whether the provided plain password matches the hashed password
    ///
    /// # Errors
    ///
    /// when could not verify password
    #[must_use]
    pub fn verify_password(&self, password: &str) -> bool {
        hash::verify_password(password, &self.password)
    }

    /// Asynchronously creates a user with a password and saves it to the
    /// database.
    ///
    /// # Errors
    ///
    /// When could not save the user into the DB
    pub async fn create_with_password(db: &PgPool, params: &RegisterParams) -> ModelResult<Self> {
        let password_hash =
            hash::hash_password(&params.password).map_err(|e| ModelError::Any(e.into()))?;
        let id = Uuid::now_v7();
        let api_key = Uuid::new_v4();
        let user = Self {
            id,
            email: params.email.to_string(),
            password: password_hash,
            api_key,
            username: params.username.to_string(),
            ..Default::default()
        };
        user.validate()?;

        let user = sqlx::query_as!(
            Self,
            "INSERT INTO users (id, email, password, api_key, username, reset_token, \
             reset_sent_at, email_verification_token, email_verification_sent_at, \
             email_verified_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *",
            user.id,
            user.email,
            user.password,
            user.api_key,
            user.username,
            user.reset_token,
            user.reset_sent_at,
            user.email_verification_token,
            user.email_verification_sent_at,
            user.email_verified_at
        )
        .fetch_one(db)
        .await?;

        Ok(user)
    }

    /// Creates a JWT
    ///
    /// # Errors
    ///
    /// when could not convert user claims to jwt token
    pub fn generate_jwt(&self, secret: &str, expiration: &u64) -> ModelResult<String> {
        Ok(jwt::JWT::new(secret).generate_token(expiration, self.id.to_string(), None)?)
    }

    /// Sets the email verification information for the user and
    /// updates it in the database.
    ///
    /// This method is used to record the timestamp when the email verification
    /// was sent and generate a unique verification token for the user.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_email_verification_sent(&mut self, db: &PgPool) -> ModelResult<()> {
        self.email_verification_sent_at = Some(Utc::now().naive_utc());
        self.email_verification_token = Some(Uuid::new_v4());
        self.updated_at = Utc::now().naive_utc();
        sqlx::query!(
            "UPDATE users SET email_verification_sent_at = $1, email_verification_token = $2, \
             updated_at = $3",
            self.email_verification_sent_at,
            self.email_verification_token,
            self.updated_at
        )
        .execute(db)
        .await?;
        Ok(())
    }

    /// Sets the information for a reset password request,
    /// generates a unique reset password token, and updates it in the
    /// database.
    ///
    /// This method records the timestamp when the reset password token is sent
    /// and generates a unique token for the user.
    ///
    /// # Arguments
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn set_forgot_password_sent(&mut self, db: &PgPool) -> ModelResult<()> {
        self.reset_sent_at = Some(Utc::now().naive_utc());
        self.reset_token = Some(Uuid::new_v4());
        self.updated_at = Utc::now().naive_utc();
        sqlx::query!(
            "UPDATE users SET reset_sent_at = $1, reset_token = $2, updated_at = $3",
            self.reset_sent_at,
            self.reset_token,
            self.updated_at
        )
        .execute(db)
        .await?;
        Ok(())
    }

    /// Records the verification time when a user verifies their
    /// email and updates it in the database.
    ///
    /// This method sets the timestamp when the user successfully verifies their
    /// email.
    ///
    /// # Errors
    ///
    /// when has DB query error
    pub async fn verified(&mut self, db: &PgPool) -> ModelResult<()> {
        self.email_verified_at = Some(Utc::now().naive_utc());
        self.updated_at = Utc::now().naive_utc();
        sqlx::query!(
            "UPDATE users SET email_verified_at = $1, updated_at = $2",
            self.email_verified_at,
            self.updated_at
        )
        .execute(db)
        .await?;
        Ok(())
    }

    /// Resets the current user password with a new password and
    /// updates it in the database.
    ///
    /// This method hashes the provided password and sets it as the new password
    /// for the user.
    /// # Errors
    ///
    /// when has DB query error or could not hashed the given password
    pub async fn reset_password(&mut self, db: &PgPool, password: &str) -> ModelResult<()> {
        self.password = hash::hash_password(password).map_err(|e| ModelError::Any(e.into()))?;
        self.reset_token = None;
        self.reset_sent_at = None;
        self.updated_at = Utc::now().naive_utc();
        sqlx::query!(
            "UPDATE users SET password = $1, reset_token = $2, reset_sent_at = $3, updated_at = $4",
            self.password,
            self.reset_token,
            self.reset_sent_at,
            self.updated_at
        )
        .execute(db)
        .await?;
        Ok(())
    }
}
