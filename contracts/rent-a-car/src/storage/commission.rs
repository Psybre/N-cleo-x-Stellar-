use soroban_sdk::Env;
use crate::storage::types::{storage::DataKey, error::Error};

pub fn read_commission(env: &Env) -> Result<i128, Error> {
    env.storage()
        .persistent()
        .get(&DataKey::Commission)
        .ok_or(Error::CommissionNotSet)
}

pub fn write_commission(env: &Env, amount: &i128) {
    env.storage()
        .persistent()
        .set(&DataKey::Commission, amount);
}

pub fn read_commission_balance(env: &Env) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::CommissionBalance)
        .unwrap_or(0)  
}

pub fn write_commission_balance(env: &Env, amount: &i128) {
    env.storage()
        .persistent()
        .set(&DataKey::CommissionBalance, amount);
}