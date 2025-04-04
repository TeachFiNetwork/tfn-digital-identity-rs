#![no_std]

multiversx_sc::imports!();

pub mod common;

use common::{config::*, errors::*};

#[multiversx_sc::contract]
pub trait TFNDigitalIdentityContract<ContractReader>:
common::config::ConfigModule
{
    #[init]
    fn init(&self) {
        self.set_state_active();
    }

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(addIdentity)]
    fn add_identity(
        &self,
        is_corporate: bool,
        legal_id: BigUint,
        birthdate: u64,
        wallet: ManagedAddress,
        name: ManagedBuffer,
        description: ManagedBuffer,
        contact: ManagedVec<ManagedBuffer>,
    ) -> u64 {
        require!(self.state().get() == State::Active, ERROR_NOT_ACTIVE);
        require!(self.get_identity_by_wallet(&wallet).is_none(), ERROR_WALLET_ALREADY_REGISTERED);
        require!(self.get_identity_by_legal_id(&legal_id).is_none(), ERROR_LEGAL_ID_ALREADY_REGISTERED);

        let id = self.last_identity_id().get();
        let identity = Identity {
            id,
            is_corporate,
            legal_id,
            birthdate,
            wallet,
            name,
            description,
            contact,
        };
        self.identities(identity.id).set(&identity);
        self.last_identity_id().set(id + 1);

        id
    }

    #[endpoint(editIdentity)]
    fn edit_identity(&self, new_identity: Identity<Self::Api>) {
        require!(self.state().get() == State::Active, ERROR_NOT_ACTIVE);
        self.only_owner_or_wallet(new_identity.id);

        let mut old_identity = self.identities(new_identity.id).get();
        if self.blockchain().get_caller() == self.blockchain().get_owner_address() {
            old_identity.wallet = new_identity.wallet;
        }
        self.identities(new_identity.id).set(Identity{
            id: new_identity.id,
            is_corporate: new_identity.is_corporate,
            legal_id: new_identity.legal_id,
            birthdate: new_identity.birthdate,
            wallet: old_identity.wallet,
            name: new_identity.name,
            description: new_identity.description,
            contact: new_identity.contact,
        });
    }

    #[endpoint(removeIdentity)]
    fn remove_identity(&self, identity_id: u64) {
        require!(self.state().get() == State::Active, ERROR_NOT_ACTIVE);
        self.only_owner_or_wallet(identity_id);

        let mut links = self.get_children_identities(identity_id);
        links.append_vec(self.get_parent_identities(identity_id));

        require!(links.is_empty(), ERROR_NOT_ALLOWED);
        // for link in links.iter() {
        //     self.remove_identity_link(link.id);
        // }

        for key in self.identity_keys(identity_id).iter() {
            for value_id in 0..self.last_identity_key_id(identity_id, &key).get() {
                self.identity_key_value(identity_id, &key, value_id).clear();
            }
            self.last_identity_key_id(identity_id, &key).clear();
            self.identity_key_modifiers(identity_id, &key).clear();
        }
        self.identity_keys(identity_id).clear();
        self.identities(identity_id).clear();
        if identity_id == self.last_identity_id().get() - 1 {
            self.last_identity_id().set(identity_id);
        }
    }

    #[endpoint(addIdentityLink)]
    fn add_identity_link(
        &self,
        parent_id: u64,
        child_id: u64,
        relation: ManagedBuffer,
        keys: OptionalValue<ManagedVec<ManagedBuffer>>,
    ) -> u64 {
        require!(self.state().get() == State::Active, ERROR_NOT_ACTIVE);
        require!(child_id != parent_id, ERROR_NOT_ALLOWED);
        require!(!relation.is_empty(), ERROR_EMPTY_VALUE);
        require!(!self.identities(parent_id).is_empty(), ERROR_IDENTITY_NOT_FOUND);
        require!(!self.identities(child_id).is_empty(), ERROR_IDENTITY_NOT_FOUND);

        let owner = self.blockchain().get_owner_address();
        let caller = self.blockchain().get_caller();
        let parent_identity = self.identities(parent_id).get();
        require!(caller == owner || caller == parent_identity.wallet, ERROR_NOT_ALLOWED);

        let link_id = self.last_identity_link_id().get();
        let link = IdentityLink {
            id: link_id,
            child_id,
            parent_id,
            relation,
        };
        self.identity_links(link_id).set(&link);
        self.last_identity_link_id().set(link_id + 1);
        self.child_links(child_id).insert(link_id);
        self.parent_links(parent_id).insert(link_id);

        if keys.is_some() {
            for key in keys.into_option().unwrap().into_iter() {
                self.add_modifier(child_id, &key, OptionalValue::Some(parent_identity.wallet.clone()));
            }
        }

        link_id
    }

    #[endpoint(removeIdentityLink)]
    fn remove_identity_link(&self, link_id: u64) {
        require!(self.state().get() == State::Active, ERROR_NOT_ACTIVE);
        require!(!self.identity_links(link_id).is_empty(), ERROR_LINK_NOT_FOUND);

        let link = self.identity_links(link_id).get();
        let owner = self.blockchain().get_owner_address();
        let caller = self.blockchain().get_caller();
        let parent_identity = self.identities(link.parent_id).get();
        require!(caller == owner || caller == parent_identity.wallet, ERROR_NOT_ALLOWED);

        self.child_links(link.child_id).swap_remove(&link.id);
        self.parent_links(link.parent_id).swap_remove(&link.id);
        self.identity_links(link.id).clear();
        if link.id == self.last_identity_link_id().get() - 1 {
            self.last_identity_link_id().set(link.id);
        }
    }

    #[endpoint(addModifier)]
    fn add_modifier(
        &self,
        identity_id: u64,
        key: &ManagedBuffer,
        opt_modifier: OptionalValue<ManagedAddress>,
    ) {
        require!(self.state().get() == State::Active, ERROR_NOT_ACTIVE);
        require!(!key.is_empty(), ERROR_EMPTY_VALUE);
        require!(!self.identities(identity_id).is_empty(), ERROR_IDENTITY_NOT_FOUND);

        let modifier = match opt_modifier {
            OptionalValue::Some(modifier) => modifier,
            OptionalValue::None => self.blockchain().get_caller(),
        };
        if self.identity_key_modifiers(identity_id, key).contains(&modifier) {
            return
        }

        let owner = self.blockchain().get_owner_address();
        let caller = self.blockchain().get_caller();
        let is_parent = match self.get_identity_by_wallet(&caller) {
            Some(parent) => self.is_parent_of(parent.id, identity_id),
            None => false,
        };
        require!(caller == owner || is_parent, ERROR_NOT_ALLOWED);

        if !self.identity_key_modifiers(identity_id, key).is_empty() {
            require!(caller == owner, ERROR_NOT_ALLOWED);
        }
        self.identity_key_modifiers(identity_id, key).insert(modifier);
    }

    #[endpoint(removeModifier)]
    fn remove_modifier(
        &self,
        identity_id: u64,
        key: &ManagedBuffer,
        opt_modifier: OptionalValue<ManagedAddress>,
    ) {
        require!(self.state().get() == State::Active, ERROR_NOT_ACTIVE);
        require!(!self.identities(identity_id).is_empty(), ERROR_IDENTITY_NOT_FOUND);

        let modifier = match opt_modifier {
            OptionalValue::Some(modifier) => modifier,
            OptionalValue::None => self.blockchain().get_caller(),
        };
        if !self.identity_key_modifiers(identity_id, key).contains(&modifier) {
            return
        }

        let owner = self.blockchain().get_owner_address();
        let caller = self.blockchain().get_caller();
        require!(caller == owner || caller == modifier, ERROR_NOT_ALLOWED);

        self.identity_key_modifiers(identity_id, key).swap_remove(&modifier);
    }

    #[endpoint(addIdentityKeyValue)]
    fn add_identity_key_value(
        &self,
        identity_id: u64,
        key: ManagedBuffer,
        value: ManagedBuffer,
    ) -> u64 {
        require!(self.state().get() == State::Active, ERROR_NOT_ACTIVE);
        self.only_modifier(identity_id, &key);

        let value_id = self.last_identity_key_id(identity_id, &key).get();
        self.identity_key_value(identity_id, &key, value_id).set(Value{
            id: value_id,
            value,
            modifier: self.blockchain().get_caller(),
            timestamp: self.blockchain().get_block_timestamp(),
        });
        self.last_identity_key_id(identity_id, &key).set(value_id + 1);
        self.identity_keys(identity_id).insert(key);

        value_id
    }

    #[endpoint(editIdentityKeyValue)]
    fn edit_identity_key_value(
        &self,
        identity_id: u64,
        key: ManagedBuffer,
        value_id: u64,
        new_value: ManagedBuffer,
    ) {
        require!(self.state().get() == State::Active, ERROR_NOT_ACTIVE);
        self.only_modifier(identity_id, &key);
        require!(self.identity_keys(identity_id).contains(&key), ERROR_KEY_NOT_FOUND);
        require!(!self.identity_key_value(identity_id, &key, value_id).is_empty(), ERROR_VALUE_NOT_FOUND);

        let old_value = self.identity_key_value(identity_id, &key, value_id).get();
        let owner = self.blockchain().get_owner_address();
        let caller = self.blockchain().get_caller();
        require!(caller == owner || caller == old_value.modifier, ERROR_NOT_ALLOWED);

        self.identity_key_value(identity_id, &key, value_id).set(Value{
            id: value_id,
            value: new_value,
            modifier: old_value.modifier,
            timestamp: old_value.timestamp,
        });
    }

    #[endpoint(removeIdentityKeyValue)]
    fn remove_identity_key_value(
        &self,
        identity_id: u64,
        key: ManagedBuffer,
        value_id: u64,
    ) {
        require!(self.state().get() == State::Active, ERROR_NOT_ACTIVE);
        self.only_modifier(identity_id, &key);
        require!(self.identity_keys(identity_id).contains(&key), ERROR_KEY_NOT_FOUND);
        require!(!self.identity_key_value(identity_id, &key, value_id).is_empty(), ERROR_VALUE_NOT_FOUND);

        self.identity_key_value(identity_id, &key, value_id).clear();
        if value_id == self.last_identity_key_id(identity_id, &key).get() - 1 {
            self.last_identity_key_id(identity_id, &key).set(value_id);
        }
    }

    // helpers
    fn only_modifier(&self, identity_id: u64, key: &ManagedBuffer) {
        require!(!self.identities(identity_id).is_empty(), ERROR_IDENTITY_NOT_FOUND);

        let owner = self.blockchain().get_owner_address();
        let caller = self.blockchain().get_caller();
        require!(caller == owner || self.identity_key_modifiers(identity_id, key).contains(&caller), ERROR_NOT_ALLOWED);
    }
    
    fn only_owner_or_wallet(&self, identity_id: u64) {
        require!(!self.identities(identity_id).is_empty(), ERROR_IDENTITY_NOT_FOUND);

        let owner = self.blockchain().get_owner_address();
        let caller = self.blockchain().get_caller();
        let identity = self.identities(identity_id).get();
        require!(caller == owner || caller == identity.wallet, ERROR_NOT_ALLOWED);
    }
}
