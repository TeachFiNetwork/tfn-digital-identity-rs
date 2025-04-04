multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Copy, Clone, Debug)]
pub enum State {
    Inactive,
    Active,
}

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Clone, Debug)]
pub struct Identity<M: ManagedTypeApi> {
    pub id: u64,
    pub is_corporate: bool,
    pub legal_id: BigUint<M>, // CNP
    pub wallet: ManagedAddress<M>,
    pub name: ManagedBuffer<M>,
    pub description: ManagedBuffer<M>,
    pub contact: ManagedVec<M, ManagedBuffer<M>>,
    pub link_ids: ManagedVec<M, u64>,
}

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Clone, Debug)]
pub struct Value<M: ManagedTypeApi> {
    pub id: u64,
    pub value: ManagedBuffer<M>,
    pub modifier: ManagedAddress<M>,
    pub timestamp: u64,
}

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Clone, Debug)]
pub struct IdentityLink<M: ManagedTypeApi> {
    pub id: u64,
    pub child_id: u64,
    pub parent_id: u64,
    pub relation: ManagedBuffer<M>,
}

#[multiversx_sc::module]
pub trait ConfigModule {
    // state
    #[only_owner]
    #[endpoint(setStateActive)]
    fn set_state_active(&self) {
        self.state().set(State::Active);
    }

    #[only_owner]
    #[endpoint(setStateInactive)]
    fn set_state_inactive(&self) {
        self.state().set(State::Inactive);
    }

    #[view(getState)]
    #[storage_mapper("state")]
    fn state(&self) -> SingleValueMapper<State>;

    // identities
    #[view(getIdentity)]
    #[storage_mapper("identities")]
    fn identities(&self, identity_id: u64) -> SingleValueMapper<Identity<Self::Api>>;

    #[view(getLastIdentityId)]
    #[storage_mapper("last_identity_id")]
    fn last_identity_id(&self) -> SingleValueMapper<u64>;

    #[view(getIdentityKeyModifiers)]
    #[storage_mapper("identity_key_modifiers")]
    fn identity_key_modifiers(&self, identity_id: u64, key: &ManagedBuffer) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

    // keys and values
    #[view(getIdentityKeys)]
    #[storage_mapper("identity_keys")]
    fn identity_keys(&self, identity_id: u64) -> UnorderedSetMapper<ManagedBuffer<Self::Api>>;

    #[view(getLastIdentityKeyId)]
    #[storage_mapper("last_identity_key_id")]
    fn last_identity_key_id(&self, identity_id: u64, key: &ManagedBuffer) -> SingleValueMapper<u64>;

    #[view(getIdentityKeyValue)]
    #[storage_mapper("identity_key_value")]
    fn identity_key_value(&self, identity_id: u64, key: &ManagedBuffer, value_id: u64) -> SingleValueMapper<Value<Self::Api>>;

    // identity links
    #[view(getIdentityLink)]
    #[storage_mapper("identity_links")]
    fn identity_links(&self, link_id: u64) -> SingleValueMapper<IdentityLink<Self::Api>>;

    #[view(getLastIdentityLinkId)]
    #[storage_mapper("last_identity_link_id")]
    fn last_identity_link_id(&self) -> SingleValueMapper<u64>;

    #[view(getChildrenLinks)]
    #[storage_mapper("children_links")]
    fn children_links(&self, child_id: u64) -> UnorderedSetMapper<u64>;

    #[view(getParentLinks)]
    #[storage_mapper("parent_links")]
    fn parent_links(&self, parent_id: u64) -> UnorderedSetMapper<u64>;

    // views
    #[view(getIdentityByWallet)]
    fn get_identity_by_wallet(&self, wallet: &ManagedAddress) -> Option<u64> {
        for id in 0..self.last_identity_id().get() {
            if self.identities(id).is_empty() {
                continue;
            }

            let identity = self.identities(id).get();
            if &identity.wallet == wallet {
                return Some(identity.id);
            }
        }

        None
    }

    #[view(getIdentityByLegalId)]
    fn get_identity_by_legal_id(&self, legal_id: &BigUint) -> Option<u64> {
        if legal_id == &0 {
            return None;
        }

        for id in 0..self.last_identity_id().get() {
            if self.identities(id).is_empty() {
                continue;
            }

            let identity = self.identities(id).get();
            if &identity.legal_id == legal_id {
                return Some(identity.id);
            }
        }

        None
    }

    #[view(getIdentityKeyValues)]
    fn get_identity_key_values(&self, identity_id: u64, key: &ManagedBuffer) -> ManagedVec<Value<Self::Api>> {
        let mut values = ManagedVec::new();
        for value_id in 0..self.last_identity_key_id(identity_id, key).get() {
            if !self.identity_key_value(identity_id, key, value_id).is_empty() {
                values.push(self.identity_key_value(identity_id, key, value_id).get());
            }
        }

        values
    }

    #[view(getChildrenIdentities)]
    fn get_children_identities(&self, parent_id: u64) -> ManagedVec<Identity<Self::Api>> {
        let mut children = ManagedVec::new();
        for link_id in self.parent_links(parent_id).iter() {
            let link = self.identity_links(link_id).get();
            children.push(self.identities(link.child_id).get());
        }

        children
    }

    #[view(getParentIdentities)]
    fn get_parent_identities(&self, child_id: u64) -> ManagedVec<Identity<Self::Api>> {
        let mut parents = ManagedVec::new();
        for link_id in self.children_links(child_id).iter() {
            let link = self.identity_links(link_id).get();
            parents.push(self.identities(link.parent_id).get());
        }

        parents
    }

    #[view(isParentOf)]
    fn is_parent_of(&self, parent_id: u64, child_id: u64) -> bool {
        for link_id in self.children_links(child_id).iter() {
            let link = self.identity_links(link_id).get();
            if link.parent_id == parent_id {
                return true;
            }
        }

        false
    }
}
