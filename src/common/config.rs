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
    pub birthdate: u64,
    pub address: ManagedAddress<M>,
    pub name: ManagedBuffer<M>,
    pub description: ManagedBuffer<M>,
    pub image: ManagedBuffer<M>,
    pub contact: ManagedVec<M, ManagedBuffer<M>>,
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

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Clone, Debug)]
pub struct LinkRequest<M: ManagedTypeApi> {
    pub id: u64,
    pub parent_id: u64,
    pub child_id: u64,
    pub relation: ManagedBuffer<M>,
    pub keys: ManagedVec<M, ManagedBuffer<M>>,
}

#[type_abi]
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Clone, Debug)]
pub struct UnlinkRequest<M: ManagedTypeApi> {
    pub id: u64,
    pub parent_id: u64,
    pub child_id: u64,
    pub reason: ManagedBuffer<M>,
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
    fn child_links(&self, child_id: u64) -> UnorderedSetMapper<u64>;

    #[view(getParentLinks)]
    #[storage_mapper("parent_links")]
    fn parent_links(&self, parent_id: u64) -> UnorderedSetMapper<u64>;

    // requests
    #[view(getLinkRequest)]
    #[storage_mapper("link_requests")]
    fn link_requests(&self, request_id: u64) -> SingleValueMapper<LinkRequest<Self::Api>>;

    #[view(getLastListRequestId)]
    #[storage_mapper("last_link_request_id")]
    fn last_link_request_id(&self) -> SingleValueMapper<u64>;

    #[view(getUnLinkRequest)]
    #[storage_mapper("unlink_requests")]
    fn unlink_requests(&self, request_id: u64) -> SingleValueMapper<UnlinkRequest<Self::Api>>;

    #[view(getLastUnLinkRequestId)]
    #[storage_mapper("last_unlink_request_id")]
    fn last_unlink_request_id(&self) -> SingleValueMapper<u64>;

    // views
    #[view(getIdentityByWallet)]
    fn get_identity_by_wallet(&self, wallet: &ManagedAddress) -> Option<Identity<Self::Api>> {
        for id in 0..self.last_identity_id().get() {
            if self.identities(id).is_empty() {
                continue;
            }

            let identity = self.identities(id).get();
            if &identity.address == wallet {
                return Some(identity);
            }
        }

        None
    }

    #[view(getIdentityByLegalId)]
    fn get_identity_by_legal_id(&self, legal_id: &BigUint) -> Option<Identity<Self::Api>> {
        if legal_id == &0 {
            return None;
        }

        for id in 0..self.last_identity_id().get() {
            if self.identities(id).is_empty() {
                continue;
            }

            let identity = self.identities(id).get();
            if &identity.legal_id == legal_id {
                return Some(identity);
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

    #[view(getChildrenIdentitiesOfParent)]
    fn get_children_identities(&self, parent_id: u64) -> ManagedVec<Identity<Self::Api>> {
        let mut children = ManagedVec::new();
        for link_id in self.parent_links(parent_id).iter() {
            let link = self.identity_links(link_id).get();
            children.push(self.identities(link.child_id).get());
        }

        children
    }

    #[view(getParentIdentitiesOfChild)]
    fn get_parent_identities(&self, child_id: u64) -> ManagedVec<Identity<Self::Api>> {
        let mut parents = ManagedVec::new();
        for link_id in self.child_links(child_id).iter() {
            let link = self.identity_links(link_id).get();
            parents.push(self.identities(link.parent_id).get());
        }

        parents
    }

    #[view(getLinkByIds)]
    fn get_link_by_ids(&self, parent_id: u64, child_id: u64) -> Option<IdentityLink<Self::Api>> {
        for link_id in self.child_links(child_id).iter() {
            let link = self.identity_links(link_id).get();
            if link.parent_id == parent_id {
                return Some(link);
            }
        }

        None
    }

    #[view(getLinkedIdentities)]
    fn get_linked_identities(&self, identity_id: u64) -> MultiValueEncoded<Self::Api, (Identity<Self::Api>, IdentityLink<Self::Api>)> {
        let mut linked_identities = MultiValueEncoded::new();
        for link_id in self.child_links(identity_id).iter() {
            let link = self.identity_links(link_id).get();
            let parent = self.identities(link.parent_id).get();
            linked_identities.push((parent, link));
        }
        for link_id in self.parent_links(identity_id).iter() {
            let link = self.identity_links(link_id).get();
            let child = self.identities(link.child_id).get();
            linked_identities.push((child, link));
        }

        linked_identities
    }

    #[view(isParentOfChild)]
    fn is_parent_of(&self, parent_id: u64, child_id: u64) -> bool {
        for link_id in self.child_links(child_id).iter() {
            let link = self.identity_links(link_id).get();
            if link.parent_id == parent_id {
                return true;
            }
        }

        false
    }

    #[view(isChildOfParent)]
    fn is_child_of(&self, child_id: u64, parent_id: u64) -> bool {
        for link_id in self.parent_links(parent_id).iter() {
            let link = self.identity_links(link_id).get();
            if link.child_id == child_id {
                return true;
            }
        }

        false
    }

    #[view(getChildrenWithSameLastValue)]
    fn get_children_with_same_last_value(
        &self,
        parent_id: u64,
        key: ManagedBuffer,
        last_value: ManagedBuffer,
        opt_relation: OptionalValue<ManagedBuffer>,
    ) -> ManagedVec<Identity<Self::Api>> {
        let mut children = ManagedVec::new();
        let (all, relation) = match opt_relation {
            OptionalValue::Some(r) => (false, r),
            OptionalValue::None => (true, ManagedBuffer::new()),
        };
        for link_id in self.parent_links(parent_id).iter() {
            let link = self.identity_links(link_id).get();
            if !all && relation != link.relation {
                continue
            }

            let child = self.identities(link.child_id).get();
            let last_id = self.last_identity_key_id(child.id, &key).get();
            if last_id == 0 {
                continue;
            }

            let value = self.identity_key_value(child.id, &key, last_id - 1).get();
            if value.value == last_value {
                children.push(child);
            }
        }

        children
    }

    #[view(getMultipleIdentities)]
    fn get_multiple_identities(&self, identity_ids: ManagedVec<u64>) -> ManagedVec<Identity<Self::Api>> {
        let mut identities = ManagedVec::new();
        for id in identity_ids.into_iter() {
            if !self.identities(id).is_empty() {
                identities.push(self.identities(id).get());
            }
        }

        identities
    }

    // requests
    #[view(getLinkRequestsByParent)]
    fn get_link_requests_by_parent(&self, parent_id: u64) -> ManagedVec<LinkRequest<Self::Api>> {
        let mut requests = ManagedVec::new();
        for request_id in 0..self.last_link_request_id().get() {
            if self.link_requests(request_id).is_empty() {
                continue;
            }

            let request = self.link_requests(request_id).get();
            if request.parent_id == parent_id {
                requests.push(request);
            }
        }

        requests
    }

    #[view(getLinkRequestsByChild)]
    fn get_link_requests_by_child(&self, child_id: u64) -> ManagedVec<LinkRequest<Self::Api>> {
        let mut requests = ManagedVec::new();
        for request_id in 0..self.last_link_request_id().get() {
            if self.link_requests(request_id).is_empty() {
                continue;
            }

            let request = self.link_requests(request_id).get();
            if request.child_id == child_id {
                requests.push(request);
            }
        }

        requests
    }

    #[view(getUnlinkRequestsByParent)]
    fn get_unlink_requests_by_parent(&self, parent_id: u64) -> ManagedVec<UnlinkRequest<Self::Api>> {
        let mut requests = ManagedVec::new();
        for request_id in 0..self.last_unlink_request_id().get() {
            if self.unlink_requests(request_id).is_empty() {
                continue;
            }

            let request = self.unlink_requests(request_id).get();
            if request.parent_id == parent_id {
                requests.push(request);
            }
        }

        requests
    }

    #[view(getUnlinkRequestsByChild)]
    fn get_unlink_requests_by_child(&self, child_id: u64) -> ManagedVec<UnlinkRequest<Self::Api>> {
        let mut requests = ManagedVec::new();
        for request_id in 0..self.last_unlink_request_id().get() {
            if self.unlink_requests(request_id).is_empty() {
                continue;
            }

            let request = self.unlink_requests(request_id).get();
            if request.child_id == child_id {
                requests.push(request);
            }
        }

        requests
    }
}
