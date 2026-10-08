#![no_std]

use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, Address, Env,
};

/// Contract errors. Numbers are part of the ABI and must never change.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum CounterError {
    /// `initialize` was called on an already-initialized instance.
    AlreadyInitialized = 1,
    /// A state-changing call ran before `initialize`.
    NotInitialized = 2,
    /// The caller did not match the owner.
    Unauthorized = 3,
}

/// Storage layout.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Persistent — the current count (`u32`).
    Count,
    /// Instance — the only address permitted to mutate (`Address`).
    Owner,
}

/// Emitted by `initialize`. Marks the moment the contract was bound.
#[contractevent]
#[derive(Clone)]
pub struct Initialized {
    #[topic]
    pub owner: Address,
}

/// Emitted by `increment`. `value` is the count after the increment.
#[contractevent]
#[derive(Clone)]
pub struct Incremented {
    #[topic]
    pub owner: Address,
    pub value: u32,
}

/// Emitted by `set`.
#[contractevent]
#[derive(Clone)]
pub struct CountSet {
    #[topic]
    pub owner: Address,
    pub value: u32,
}

/// Emitted by `reset`.
#[contractevent]
#[derive(Clone)]
pub struct Reset {
    #[topic]
    pub owner: Address,
}

/// How long instance storage is extended on each mutation, in ledgers.
/// 1 day ≈ 17280 ledgers; the threshold keeps rent payments infrequent
/// while the contract is in use.
const INSTANCE_THRESHOLD: u32 = 100;
const INSTANCE_EXTEND_TO: u32 = 17_280;

#[contract]
pub struct Counter;

#[contractimpl]
impl Counter {
    /// Bind the contract to an owner. Callable exactly once.
    ///
    /// Auth: none — the first caller becomes the owner, which is the usual
    /// pattern for a one-shot constructor.
    pub fn initialize(env: Env, owner: Address) -> Result<(), CounterError> {
        if env.storage().instance().has(&DataKey::Owner) {
            return Err(CounterError::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().persistent().set(&DataKey::Count, &0u32);
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_THRESHOLD, INSTANCE_EXTEND_TO);

        Initialized {
            owner: owner.clone(),
        }
        .publish(&env);

        Ok(())
    }

    /// Add one to the count and return the new value.
    ///
    /// Auth: the owner must authorize the transaction.
    pub fn increment(env: Env) -> Result<u32, CounterError> {
        let owner = owner_of(&env)?;
        owner.require_auth();

        let next = read_count(&env).saturating_add(1);
        write_count(&env, next);

        Incremented { owner, value: next }.publish(&env);

        Ok(next)
    }

    /// Overwrite the count. Exists so a caller can correct a value instead of
    /// resetting and re-incrementing.
    ///
    /// Auth: the owner must authorize the transaction.
    pub fn set(env: Env, value: u32) -> Result<u32, CounterError> {
        let owner = owner_of(&env)?;
        owner.require_auth();

        write_count(&env, value);

        CountSet { owner, value }.publish(&env);

        Ok(value)
    }

    /// Return the count to zero.
    ///
    /// Auth: the owner must authorize the transaction.
    pub fn reset(env: Env) -> Result<(), CounterError> {
        let owner = owner_of(&env)?;
        owner.require_auth();

        write_count(&env, 0);

        Reset { owner }.publish(&env);

        Ok(())
    }

    /// Read the current count. Total function: an uninitialized contract
    /// reports `0` rather than erroring, so read-only callers never need to
    /// special-case a fresh deployment.
    pub fn get(env: Env) -> u32 {
        read_count(&env)
    }

    /// Read the bound owner.
    pub fn owner(env: Env) -> Result<Address, CounterError> {
        owner_of(&env)
    }
}

fn owner_of(env: &Env) -> Result<Address, CounterError> {
    env.storage()
        .instance()
        .get(&DataKey::Owner)
        .ok_or(CounterError::NotInitialized)
}

fn read_count(env: &Env) -> u32 {
    env.storage().persistent().get(&DataKey::Count).unwrap_or(0)
}

fn write_count(env: &Env, value: u32) {
    env.storage().persistent().set(&DataKey::Count, &value);
    env.storage()
        .persistent()
        .extend_ttl(&DataKey::Count, INSTANCE_THRESHOLD, INSTANCE_EXTEND_TO);
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_THRESHOLD, INSTANCE_EXTEND_TO);
}

#[cfg(test)]
mod test;
