

use soroban_sdk::{Env};
use crate::methods::token::token::token_transfer;
use crate::storage::{
    admin::read_admin,
    commission::{read_commission_balance, write_commission_balance},
    contract_balance::{read_contract_balance, write_contract_balance},
    types::error::Error
};

pub fn withdraw_commission(env: &Env, amount: i128) -> Result<(), Error> {
    let admin = read_admin(env)?;
    admin.require_auth();
    
    if amount <= 0 {
        return Err(Error::AmountMustBePositive);
    }

    let commission_balance = read_commission_balance(env);

    if amount > commission_balance {
        return Err(Error::InsufficientCommissionBalance);
    }

    let mut contract_balance = read_contract_balance(env);

    if amount > contract_balance {
        return Err(Error::BalanceNotAvailableForAmountRequested);
    }

    token_transfer(env, &env.current_contract_address(), &admin, &amount)?;

    let new_commission_balance = commission_balance.checked_sub(amount).ok_or(Error::UnderflowError)?;
    contract_balance = contract_balance.checked_sub(amount).ok_or(Error::UnderflowError)?;

    write_commission_balance(env, &new_commission_balance);
    write_contract_balance(env, &contract_balance);

    Ok(())
}
