use qrcode::{types::QrError, QrCode};
use totp_rs::{Secret, TOTP};

use crate::model::auth::Auth;

pub struct AuthUsecase;

impl AuthUsecase {
    pub fn signup(account_name: String) -> Result<Auth, QrError> {
        let totp = TOTP::new(
            totp_rs::Algorithm::SHA256,
            6,
            1,
            30,
            Secret::generate_secret().to_bytes().unwrap(),
            Some("totp-based-2fa-server".to_string()),
            account_name,
        )
        .unwrap();
        let auth_url = totp.get_url();
        let auth_qr = QrCode::new(&auth_url.as_bytes())?;
        let auth_qr_str = auth_qr.render().light_color(" ").dark_color("#").build();

        return Ok(Auth::new(auth_url, auth_qr_str));
    }
}
