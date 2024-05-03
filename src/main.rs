use google_authenticator::GoogleAuthenticator;
use image::Luma;
use once_cell::sync::OnceCell;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use qrcode::QrCode;

static GA_AUTH: OnceCell<GoogleAuthenticator> = OnceCell::new();

fn get_ga_auth() -> &'static GoogleAuthenticator {
    GA_AUTH.get_or_init(|| GoogleAuthenticator::new())
}

pub fn create_otp_secret(length: u8) -> String {
    let ga = get_ga_auth();
    ga.create_secret(length)
}

pub fn generate_auth_url(account_name: &str, issuer_name: &str) -> String {
    let secret = create_otp_secret(32);
    let account_name = utf8_percent_encode(account_name, NON_ALPHANUMERIC);
    let issuer_name = utf8_percent_encode(issuer_name, NON_ALPHANUMERIC);
    format!(
        "otpauth://totp/{}?secret={}&issuer={}",
        account_name, secret, issuer_name
    )
}

fn main() {
    let account_name = "me";
    let issuer_name = "totp-rust";
    let qr_bits = QrCode::new(generate_auth_url(account_name, issuer_name)).unwrap();
    let qr_img = qr_bits
        .render()
        .dark_color(Luma([0u8]))
        .light_color(Luma([255u8]))
        .build();
    qr_img.save("./qrcode/qr.png").unwrap();
}
