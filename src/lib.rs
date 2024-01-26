use std::{
    fs::File,
    io::{self, Write},
    time::{SystemTime, UNIX_EPOCH},
};
use totp_rs::{Algorithm, Secret, TOTP};

pub struct TotpGenerator {
    secret: Secret,
}

pub struct TotpCode {
    pub secret: String,
    pub png: Vec<u8>,
    pub base64: String,
}

impl TotpGenerator {
    pub fn new() -> Self {
        let secret = Secret::generate_secret();
        TotpGenerator { secret }
    }

    pub fn from(secret: &str) -> Self {
        let secret = Secret::Encoded(secret.to_string());
        TotpGenerator { secret }
    }

    pub fn generate_qr_code(
        &self,
        label: &str,
        issuer: &str,
    ) -> Result<TotpCode, Box<dyn std::error::Error>> {
        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            self.secret.to_bytes().unwrap(),
            Some(label.to_string()),
            issuer.to_string(),
        )?;

        let encoded_secret = self.secret.to_encoded();
        let secret = encoded_secret.to_string();
        let png = totp.get_qr_png()?;
        let base64 = totp.get_qr_base64()?;

        Ok(TotpCode {
            secret,
            png,
            base64,
        })
    }

    pub fn verify_token(&self, token: &str) -> Result<(), &'static str> {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            self.secret.to_bytes().unwrap(),
            None,
            "".to_string(),
        )
        .unwrap();
        if totp.check(token, current_time) {
            Ok(())
        } else {
            Err("INVALID TOKEN")
        }
    }
}

pub fn save_png(png: Vec<u8>, path: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&png)?;
    Ok(())
}
