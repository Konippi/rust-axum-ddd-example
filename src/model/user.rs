use garde::Validate;

#[derive(Validate)]
pub struct User {
    #[garde(skip)]
    pub id: i64,

    #[garde(email)]
    pub email: String,

    #[garde(skip)]
    pub secret: String,
}
