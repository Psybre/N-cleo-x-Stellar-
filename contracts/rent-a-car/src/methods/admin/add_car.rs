use soroban_sdk::{Env, Address};
use crate::storage::{
    admin::read_admin,
    car::{has_car, write_car},
    structs::car::Car,
    types::{car_status::CarStatus, error::Error}
};
use crate::events;

pub fn add_car(env: &Env, owner: &Address, price_per_day: i128) -> Result<(), Error> {
    let admin = read_admin(env)?;
    admin.require_auth();
    
    if price_per_day <= 0 {
        return Err(Error::AmountMustBePositive);
    }

    if has_car(env, owner) {
        return Err(Error::CarAlreadyExist);
    }

    let car = Car {
        price_per_day,
        car_status: CarStatus::Available,
        available_to_withdraw: 0,
    };

    write_car(env, owner, &car);

    events::add_car::car_added(env, owner.clone(), price_per_day);
    Ok(())
}

