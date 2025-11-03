use soroban_sdk::{Env, Address};
use crate::storage::{
    admin::{has_admin, write_admin},
    token::write_token,
    types::error::Error
};
use crate::events;

pub fn initialize(env: &Env, admin: &Address, token: &Address) -> Result<(), Error> {
    if admin == token {
        return Err(Error::AdminTokenConflict);
    }

    if has_admin(&env) {
        return Err(Error::ContractInitialized);
    }

    write_admin(env, admin);
    write_token(env, token);

    events::contract::contract_initialized(env, admin.clone(), token.clone());

    Ok(())
}


