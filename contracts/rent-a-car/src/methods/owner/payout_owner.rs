use soroban_sdk::{Env, Address};
use crate::methods::token::token::token_transfer;
use crate::storage::types::car_status::CarStatus;
use crate::storage::{
    car::{read_car, write_car},
    contract_balance::{read_contract_balance, write_contract_balance},
    types::error::Error
};
use crate::events;

pub fn payout_owner(env: &Env, owner: &Address, amount: i128) -> Result<(), Error> {
    owner.require_auth();

    if amount <= 0 {
        return Err(Error::AmountMustBePositive);
    }

    let mut car = read_car(&env, owner)?;

    if car.car_status != CarStatus::Available {
        return Err(Error::CarMustBeReturnedFirst);
    }

    if amount > car.available_to_withdraw {
        return Err(Error::InsufficientBalance);
    }

    let mut contract_balance = read_contract_balance(&env);

    if amount > contract_balance {
        return Err(Error::BalanceNotAvailableForAmountRequested);
    }

    token_transfer(&env, &env.current_contract_address(), owner, &amount)?;

    car.available_to_withdraw = car.available_to_withdraw.checked_sub(amount).ok_or(Error::UnderflowError)?;

    contract_balance = contract_balance.checked_sub(amount).ok_or(Error::UnderflowError)?;

    write_car(&env, owner, &car);
    write_contract_balance(&env, &contract_balance);

    events::payout_owner::payout_owner(env, owner.clone(), amount);
    Ok(())
}


