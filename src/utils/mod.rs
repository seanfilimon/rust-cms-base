use actix_web::cookie::time::OffsetDateTime;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use crate::errors::MyError;
use crate::models::{Admin, User};

mod jwt_numeric_date {
    use actix_web::cookie::time::OffsetDateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let timestamp = date.unix_timestamp();
        serializer.serialize_i64(timestamp)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
        where
            D: Deserializer<'de>,
    {
        OffsetDateTime::from_unix_timestamp(i64::deserialize(deserializer)?)
            .map_err(|_| serde::de::Error::custom("invalid Unix timestamp value"))
    }

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct UserClaim {
    pub data : User,
    #[serde(with = "jwt_numeric_date")]
    pub exp: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    pub iat: OffsetDateTime,
}

impl UserClaim {
    pub fn new(data: User, exp: OffsetDateTime, iat: OffsetDateTime) -> Self {
        let iat = iat
            .date()
            .with_hms_milli(iat.hour(), iat.minute(), iat.second(), 0)
            .unwrap()
            .assume_utc();
        let exp = exp
            .date()
            .with_hms_milli(exp.hour(), exp.minute(), exp.second(), 0)
            .unwrap()
            .assume_utc();
        Self { data, exp, iat }
    }
}

pub fn user_tokens(claims: User) -> Result<(String, String), MyError> {
    let access = std::env::var("JWT_ACCESS_TOKEN_SECRET_1").unwrap();
    let refresh = std::env::var("JWT_REFRESH_TOKEN_SECRET_1").unwrap();

    let claims = UserClaim::new(claims.clone(), OffsetDateTime::from_unix_timestamp(6*60*60*60)?, OffsetDateTime::from_unix_timestamp(0)?);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(access.as_ref()),
    )?;

    let claims = UserClaim::new(claims.data, OffsetDateTime::from_unix_timestamp(15*24*60*60*60)?, OffsetDateTime::from_unix_timestamp(0)?);
    let refresh = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(refresh.as_ref()),
    )?;

    Ok((token, refresh))
}

pub fn validate_user_token(token: &str, typee: &str) -> Result<User, MyError> {
    let secret = std::env::var(format!("JWT_{}_TOKEN_SECRET_1", typee.to_ascii_uppercase())).unwrap();
    let token_data = jsonwebtoken::decode::<UserClaim>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )?;
    Ok(token_data.claims.data)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AdminClaim {
    pub data : Admin,
    #[serde(with = "jwt_numeric_date")]
    pub exp: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    pub iat: OffsetDateTime,
}

impl AdminClaim {
    pub fn new(data: Admin, exp: OffsetDateTime, iat: OffsetDateTime) -> Self {
        let iat = iat
            .date()
            .with_hms_milli(iat.hour(), iat.minute(), iat.second(), 0)
            .unwrap()
            .assume_utc();
        let exp = exp
            .date()
            .with_hms_milli(exp.hour(), exp.minute(), exp.second(), 0)
            .unwrap()
            .assume_utc();
        Self { data, exp, iat }
    }
}

pub fn admin_tokens(claims: Admin) -> Result<(String, String), MyError> {
    let access = std::env::var("JWT_ACCESS_TOKEN_SECRET_0").unwrap();
    let refresh = std::env::var("JWT_REFRESH_TOKEN_SECRET_0").unwrap();

    let claims = AdminClaim::new(claims.clone(), OffsetDateTime::from_unix_timestamp(6*60*60*60)?, OffsetDateTime::from_unix_timestamp(0)?);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(access.as_ref()),
    )?;

    let claims = AdminClaim::new(claims.data, OffsetDateTime::from_unix_timestamp(15*24*60*60*60)?, OffsetDateTime::from_unix_timestamp(0)?);
    let refresh = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(refresh.as_ref()),
    )?;

    Ok((token, refresh))
}

pub fn validate_admin_token(token: &str, typee: &str) -> Result<Admin, MyError> {
    let secret = std::env::var(format!("JWT_{}_TOKEN_SECRET_0", typee.to_ascii_uppercase())).unwrap();
    let token_data = jsonwebtoken::decode::<AdminClaim>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )?;
    Ok(token_data.claims.data)
}

