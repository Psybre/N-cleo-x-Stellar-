use soroban_sdk::{Env, Address};
use crate::methods::token::token::token_transfer;
use crate::storage::commission::{read_commission, read_commission_balance, write_commission_balance};
use crate::storage::{
    car::{read_car, write_car},
    contract_balance::{read_contract_balance, write_contract_balance},
    rental::write_rental,
    structs::rental::Rental,
    types::{car_status::CarStatus, error::Error}
};
use crate::events;

pub fn rental(env: &Env, renter: &Address, owner: &Address, total_days_to_rent: u32, amount: i128) -> Result<(), Error> {
    renter.require_auth();

    if amount <= 0 {
        return Err(Error::AmountMustBePositive);
    }

    if total_days_to_rent == 0 {
        return Err(Error::RentalDurationCannotBeZero);
    }

    if renter == owner {
        return Err(Error::SelfRentalNotAllowed);
    }

    let mut car = read_car(env, owner)?;

    if car.car_status != CarStatus::Available {
        return Err(Error::CarAlreadyRented);
    }

    let commission = read_commission(env)?;

    let total_amount = amount.checked_add(commission).ok_or(Error::OverflowError)?;

    token_transfer(env, renter, &env.current_contract_address(), &total_amount)?;

    car.car_status = CarStatus::Rented;
    car.available_to_withdraw = car.available_to_withdraw.checked_add(amount).ok_or(Error::OverflowError)?;

    let rental = Rental {
        total_days_to_rent,
        amount,
    };

    let mut contract_balance = read_contract_balance(&env);
    contract_balance = contract_balance.checked_add(amount).ok_or(Error::OverflowError)?;

    let mut commission_balance = read_commission_balance(env);
    commission_balance = commission_balance.checked_add(commission).ok_or(Error::OverflowError)?;
    
    write_contract_balance(&env, &contract_balance);
    write_commission_balance(env, &commission_balance);
    write_car(env, owner, &car);
    write_rental(env, renter, owner, &rental);

    events::rental::rented(env, renter.clone(), owner.clone(), total_days_to_rent, amount);
    Ok(())
}