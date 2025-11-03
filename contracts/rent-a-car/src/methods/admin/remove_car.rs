use soroban_sdk::{Env, Address};
use crate::storage::{
    admin::read_admin,
    car::{has_car, remove_car as remove_car_storage},
    types::error::Error
};
use crate::events;

pub fn remove_car(env: &Env, owner: &Address) -> Result<(), Error> {
    let admin = read_admin(env)?;
    admin.require_auth();

    if !has_car(env, owner) {
        return Err(Error::CarNotFound);
    }

    remove_car_storage(env, owner);

    events::remove_car::car_removed(env, owner.clone());
    Ok(())
}

