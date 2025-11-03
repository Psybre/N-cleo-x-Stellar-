use soroban_sdk::{Env};
use crate::storage::{
    admin::read_admin,
    commission::write_commission,
    types::error::Error
};

pub fn set_commission(env: &Env, commission_amount: i128) -> Result<(), Error> {
    let admin = read_admin(env)?;
    admin.require_auth();
    
    if commission_amount < 0 {
        return Err(Error::AmountMustBePositive);
    }

    write_commission(env, &commission_amount);

    Ok(())
}