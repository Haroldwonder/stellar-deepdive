#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec};

#[derive(Clone)]
#[contracttype]
pub enum Role {
    Admin,
    Operator,
    Viewer,
}

#[derive(Clone)]
#[contracttype]
pub struct Permission {
    pub role: Role,
    pub function: Symbol,
}

#[contracttype]
pub enum DataKey {
    Roles(Address),
    Permissions(Role),
}

#[contract]
pub struct AccessControl;

#[contractimpl]
impl AccessControl {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        let mut roles = Vec::new(&env);
        roles.push_back(Role::Admin);
        env.storage()
            .persistent()
            .set(&DataKey::Roles(admin), &roles);
    }

    pub fn grant_role(env: Env, caller: Address, user: Address, role: Role) {
        caller.require_auth();
        Self::require_role(&env, &caller, Role::Admin);

        let mut roles = env
            .storage()
            .persistent()
            .get::<DataKey, Vec<Role>>(&DataKey::Roles(user.clone()))
            .unwrap_or(Vec::new(&env));
        roles.push_back(role);
        env.storage()
            .persistent()
            .set(&DataKey::Roles(user), &roles);
    }

    pub fn revoke_role(env: Env, caller: Address, user: Address, role: Role) {
        caller.require_auth();
        Self::require_role(&env, &caller, Role::Admin);

        if let Some(roles) = env
            .storage()
            .persistent()
            .get::<DataKey, Vec<Role>>(&DataKey::Roles(user.clone()))
        {
            let mut new_roles = Vec::new(&env);
            for r in roles.iter() {
                if !Self::roles_equal(&r, &role) {
                    new_roles.push_back(r);
                }
            }
            env.storage()
                .persistent()
                .set(&DataKey::Roles(user), &new_roles);
        }
    }

    pub fn has_role(env: Env, user: Address, role: Role) -> bool {
        if let Some(roles) = env
            .storage()
            .persistent()
            .get::<DataKey, Vec<Role>>(&DataKey::Roles(user))
        {
            for r in roles.iter() {
                if Self::roles_equal(&r, &role) {
                    return true;
                }
            }
        }
        false
    }

    pub fn grant_permission(env: Env, caller: Address, role: Role, function: Symbol) {
        caller.require_auth();
        Self::require_role(&env, &caller, Role::Admin);

        let mut perms = env
            .storage()
            .persistent()
            .get::<DataKey, Vec<Symbol>>(&DataKey::Permissions(role.clone()))
            .unwrap_or(Vec::new(&env));
        perms.push_back(function);
        env.storage()
            .persistent()
            .set(&DataKey::Permissions(role), &perms);
    }

    pub fn check_permission(env: Env, user: Address, function: Symbol) -> bool {
        if Self::has_role(env.clone(), user.clone(), Role::Admin) {
            return true;
        }

        if let Some(roles) = env
            .storage()
            .persistent()
            .get::<DataKey, Vec<Role>>(&DataKey::Roles(user))
        {
            for role in roles.iter() {
                if let Some(perms) = env
                    .storage()
                    .persistent()
                    .get::<DataKey, Vec<Symbol>>(&DataKey::Permissions(role))
                {
                    for perm in perms.iter() {
                        if perm == function {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn require_role(env: &Env, user: &Address, role: Role) {
        if !Self::has_role(env.clone(), user.clone(), role) {
            panic!("Unauthorized: missing required role");
        }
    }

    fn roles_equal(a: &Role, b: &Role) -> bool {
        matches!(
            (a, b),
            (Role::Admin, Role::Admin)
                | (Role::Operator, Role::Operator)
                | (Role::Viewer, Role::Viewer)
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_acl() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AccessControl);
        let client = AccessControlClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let user = Address::generate(&env);

        env.mock_all_auths();

        client.initialize(&admin);
        assert!(client.has_role(&admin, &Role::Admin));

        client.grant_role(&admin, &user, &Role::Operator);
        assert!(client.has_role(&user, &Role::Operator));

        let func = symbol_short!("transfer");
        client.grant_permission(&admin, &Role::Operator, &func);
        assert!(client.check_permission(&user, &func));

        client.revoke_role(&admin, &user, &Role::Operator);
        assert!(!client.has_role(&user, &Role::Operator));
    }

    #[test]
    fn test_has_role_false_for_unknown_user() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AccessControl);
        let client = AccessControlClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let stranger = Address::generate(&env);

        env.mock_all_auths();
        client.initialize(&admin);

        assert!(!client.has_role(&stranger, &Role::Admin));
        assert!(!client.has_role(&stranger, &Role::Operator));
    }

    #[test]
    fn test_check_permission_false_without_role_or_grant() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AccessControl);
        let client = AccessControlClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let user = Address::generate(&env);

        env.mock_all_auths();
        client.initialize(&admin);

        let func = symbol_short!("transfer");

        // user has no role at all, so no permission
        assert!(!client.check_permission(&user, &func));

        // grant a role but no permission for that role/function yet
        client.grant_role(&admin, &user, &Role::Viewer);
        assert!(!client.check_permission(&user, &func));
    }

    #[test]
    fn test_multi_role_user_retains_other_roles_after_revoke() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AccessControl);
        let client = AccessControlClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let user = Address::generate(&env);

        env.mock_all_auths();
        client.initialize(&admin);

        client.grant_role(&admin, &user, &Role::Operator);
        client.grant_role(&admin, &user, &Role::Viewer);
        assert!(client.has_role(&user, &Role::Operator));
        assert!(client.has_role(&user, &Role::Viewer));

        client.revoke_role(&admin, &user, &Role::Operator);
        assert!(!client.has_role(&user, &Role::Operator));
        assert!(client.has_role(&user, &Role::Viewer));
    }

    #[test]
    fn test_admin_always_has_permission() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AccessControl);
        let client = AccessControlClient::new(&env, &contract_id);

        let admin = Address::generate(&env);

        env.mock_all_auths();
        client.initialize(&admin);

        // Admin has permission for any function, even one nobody granted explicitly.
        let func = symbol_short!("anything");
        assert!(client.check_permission(&admin, &func));
    }

    #[test]
    #[should_panic(expected = "Unauthorized: missing required role")]
    fn test_grant_role_by_non_admin_panics() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AccessControl);
        let client = AccessControlClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        let other = Address::generate(&env);

        env.mock_all_auths();
        client.initialize(&admin);

        // `user` is not an admin, so granting a role should panic.
        client.grant_role(&user, &other, &Role::Viewer);
    }
}
