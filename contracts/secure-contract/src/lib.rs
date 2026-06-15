#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, panic_with_error, symbol_short, Address, Env, Symbol,
};

mod acl {
    soroban_sdk::contractimport!(
        file = "../access-control/target/wasm32-unknown-unknown/release/access_control.wasm"
    );
}

/// Errors returned by `SecureContract`.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    /// `initialize` has not been called yet, so the ACL contract address is unknown.
    NotInitialized = 1,
    /// The caller does not have the permission required for this function.
    AccessDenied = 2,
}

#[contract]
pub struct SecureContract;

#[contractimpl]
impl SecureContract {
    pub fn initialize(env: Env, admin: Address, acl_contract: Address) {
        admin.require_auth();
        env.storage().instance().set(&symbol_short!("acl"), &acl_contract);
        env.storage().instance().set(&symbol_short!("admin"), &admin);
    }

    pub fn protected_function(env: Env, caller: Address) -> u32 {
        caller.require_auth();
        Self::check_permission(&env, &caller, symbol_short!("protected"));
        42
    }

    pub fn admin_only(env: Env, caller: Address) -> bool {
        caller.require_auth();
        Self::check_permission(&env, &caller, symbol_short!("admin"));
        true
    }

    fn check_permission(env: &Env, user: &Address, function: Symbol) {
        let acl_addr: Address = match env.storage().instance().get(&symbol_short!("acl")) {
            Some(addr) => addr,
            None => panic_with_error!(env, ContractError::NotInitialized),
        };

        let acl_client = acl::Client::new(env, &acl_addr);

        if !acl_client.check_permission(user, &function) {
            panic_with_error!(env, ContractError::AccessDenied);
        }
    }
}
