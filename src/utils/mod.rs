use actix_web::cookie::time::OffsetDateTime;
use actix_web::web::Header;
use jsonwebtoken::{encode, EncodingKey};
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Claims<T: Serialize> {
    pub data : T,
    #[serde(with = "jwt_numeric_date")]
    pub exp: OffsetDateTime,
    #[serde(with = "jwt_numeric_date")]
    pub iat: OffsetDateTime,
}

impl<T: Serialize> Claims<T> {
    pub fn new(data: T, exp: OffsetDateTime, iat: OffsetDateTime) -> Self {
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

pub fn tokens<T>(claims: T) -> std::io::Result<(String, String)> {
    let access = std::env::var("JWT_ACCESS_TOKEN_SECRET").unwrap();
    let refresh = std::env::var("JWT_REFRESH_TOKEN_SECRET").unwrap();

    let claims = Claims::new(claims, OffsetDateTime::from_unix_timestamp(6*60*60*60)?, OffsetDateTime::from_unix_timestamp(0)?);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(access.as_ref()),
    )?;

    let claims = Claims::new(claims.data, OffsetDateTime::from_unix_timestamp(15*24*60*60*60)?, OffsetDateTime::from_unix_timestamp(0)?);
    let refresh = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(refresh.as_ref()),
    )?;

    Ok((token, refresh))
}

pub fn validate_token<T>(token: &str, typee: &str) -> std::io::Result<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let secret = std::env::var(format!("JWT_{}_TOKEN_SECRET", typee.to_ascii_uppercase())).unwrap();
    let token_data = jsonwebtoken::decode::<Claims<T>>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )?;
    Ok(token_data.claims.data)
}