use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::testutils::Events as _;
use soroban_sdk::{Address, Env};

struct Fixture {
    env: Env,
    owner: Address,
    contract_id: Address,
    client: CounterClient<'static>,
}

impl Fixture {
    fn new() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let owner = Address::generate(&env);
        let contract_id = env.register(Counter, ());
        let client = CounterClient::new(&env, &contract_id);

        Self {
            env,
            owner,
            contract_id,
            client,
        }
    }

    fn initialized() -> Self {
        let f = Self::new();
        f.client.initialize(&f.owner);
        f
    }

    /// Read persistent storage from outside the contract. Storage can only be
    /// touched inside an `as_contract` scope.
    fn stored_count(&self) -> u32 {
        let contract_id = self.contract_id.clone();
        self.env
            .as_contract(&contract_id, || {
                self.env.storage().persistent().get(&DataKey::Count)
            })
            .expect("count should be persisted")
    }
}

#[test]
fn initialize_sets_owner_and_zero_count() {
    let f = Fixture::initialized();

    assert_eq!(f.client.owner(), f.owner);
    assert_eq!(f.client.get(), 0);
}

#[test]
fn initialize_twice_fails() {
    let f = Fixture::initialized();

    let second = f.client.try_initialize(&f.owner);
    assert_eq!(
        second,
        Err(Ok(CounterError::AlreadyInitialized)),
        "second initialize must be rejected"
    );
}

#[test]
fn increment_before_initialize_fails() {
    let f = Fixture::new();

    let result = f.client.try_increment();
    assert_eq!(result, Err(Ok(CounterError::NotInitialized)));
}

#[test]
fn set_before_initialize_fails() {
    let f = Fixture::new();

    assert_eq!(f.client.try_set(&5), Err(Ok(CounterError::NotInitialized)));
}

#[test]
fn reset_before_initialize_fails() {
    let f = Fixture::new();

    assert_eq!(f.client.try_reset(), Err(Ok(CounterError::NotInitialized)));
}

#[test]
fn increment_counts_up() {
    let f = Fixture::initialized();

    assert_eq!(f.client.increment(), 1);
    assert_eq!(f.client.increment(), 2);
    assert_eq!(f.client.increment(), 3);
    assert_eq!(f.client.get(), 3);
}

#[test]
fn set_overwrites_count() {
    let f = Fixture::initialized();

    assert_eq!(f.client.set(&41), 41);
    assert_eq!(f.client.increment(), 42);
    assert_eq!(f.client.get(), 42);
}

#[test]
fn reset_returns_to_zero() {
    let f = Fixture::initialized();

    f.client.increment();
    f.client.increment();
    f.client.reset();

    assert_eq!(f.client.get(), 0);
    assert_eq!(f.client.owner(), f.owner, "reset must not clear the owner");
}

#[test]
fn get_on_uninitialized_contract_reads_zero() {
    let f = Fixture::new();

    // Reads are total by design: no error, no panic, just 0.
    assert_eq!(f.client.get(), 0);
}

#[test]
fn owner_reflects_initialize_argument() {
    let f = Fixture::initialized();

    assert_eq!(f.client.owner(), f.owner);
}

#[test]
fn count_survives_multiple_transactions() {
    let f = Fixture::initialized();

    for _ in 0..10 {
        f.client.increment();
    }

    assert_eq!(f.stored_count(), 10, "persisted value must match");
    assert_eq!(f.client.get(), 10);
}

#[test]
fn initialize_emits_event() {
    let f = Fixture::new();
    f.client.initialize(&f.owner);

    let events = f.env.events().all().filter_by_contract(&f.contract_id);
    let list = events.events();

    assert_eq!(list.len(), 1, "initialize should emit exactly one event");
}

#[test]
fn increment_emits_event() {
    let f = Fixture::initialized();

    // The host exposes only the current invocation's events, so read the
    // buffer immediately after the call being asserted.
    f.client.increment();

    let events = f.env.events().all().filter_by_contract(&f.contract_id);
    let list = events.events();

    assert_eq!(list.len(), 1, "increment should emit exactly one event");
}
