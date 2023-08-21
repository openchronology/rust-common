use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env::var;

lazy_static! {
    pub static ref PGRST_HOST: String = var("PGRST_HOST").unwrap();
    pub static ref HOST_PGRST_SERVER_PORT: String = var("HOST_PGRST_SERVER_PORT").unwrap();
    pub static ref PGRST_JWT_SECRET: String = var("PGRST_JWT_SECRET").unwrap();
    pub static ref PGRST_JWT_AUD: String = var("PGRST_JWT_AUD").unwrap();

    pub static ref PGRST_JWT_KEY: Hmac<Sha256> = Hmac::new_from_slice(
        (*PGRST_JWT_SECRET).as_bytes()
    ).unwrap();
}

pub const TEST_USER_USER: &'static str = "test_user_user";
pub const TEST_MOD_USER: &'static str = "test_mod_user";
pub const TEST_ADMIN_USER: &'static str = "test_admin_user";
pub const CRON_USER: &'static str = "cron_user";
