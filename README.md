<p align="center">
  <a href="https://teachfi.network/" target="blank"><img src="https://teachfi.network/teachfi-logo.svg" width="256" alt="TeachFi Logo" /><br/>Digital Identity</a>
</p>
<br/>
<br/>
<br/>

# Description

The Digital Identity SC associates a MultiversX address with an entity's personal details, contact informations and keys-values data. 
Each identity can have multiple storage keys (imagine them as drawers) and any number of values inside each key. 
A value stores any kind of data, who added or modified it and the timestamp of the last modification.\
Parent-child type relationship links can be created between identities.\
When an identity wants to link to another, it will send a "link request" to the latter. 
The receiving identity can either accept or deny the link request. 
Once accepted, it will become a "child identity" for who requested the link, which will become the "parent identity".\
From this moment forward, the parent can create keys and add values to them in the child's identity storage. \
Any identity can have multiple parent identities. 
If a parent has created a key in the child's storage, no other parent can write to that key, unless the link with the initial parent is removed. 
If, at some point, the child identity wants to break the link, it must send an "unlink request" to the parent. Once accepted, the parent no longer has write access to the child's storage.\
Practical example: a school identity creates a link to a student's identity, then writes in its storage under the "marks" and "absences" keys. This way, the student's record will always be available on blockchain under his identity.
<br/>
<br/>
<br/>
## Endpoints

<br/>

```rust
newIdentity(
    is_corporate: bool,
    legal_id: BigUint, // for example: VAT number for corporates or ID number for people
    birthdate: u64,
    address: ManagedAddress,
    name: ManagedBuffer,
    description: ManagedBuffer,
    image: ManagedBuffer, // link to a logo or an image card
    contact: ManagedVec<ManagedBuffer>, // phone, email, telegram, twitter, facebook, etc.
) -> u64
```
>[!IMPORTANT]
>*Requirements:* state = active, `address` must not be already registered, legal_id must not be registered by another address.*

>[!NOTE]
>A new identity is created with the specified details. Returns the identity_id of the newly created identity.
<br/>

```rust
editIdentity(new_identity: Identity)
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = identity(new_identity.id).address or caller = owner.

>[!NOTE]
>Edits the identity details. Address change is possible only if the new one is not already registerd and if the identity has no active links (parent or child).
<br/>

```rust
removeIdentity(identity_id: u64)
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = identity(new_identity.id).address or caller = owner, identity has no active links.

>[!WARNING]
>Removes an identity and all its associated keys and values.
<br/>

```rust
requestLink(
    parent_id: u64,
    child_id: u64,
    relation: ManagedBuffer,
    keys: OptionalValue<ManagedVec<ManagedBuffer>>,
) -> u64
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = parent.address, any key should not be writable by another parent.

>[!NOTE]
>Creates a link request between identities `parent_id` and `child_id` with the specified `relation`. Additionally, the `keys` can be provided so the child knows in advance, what the parent intends to write in its storage.
>Returns the ID of the newly created link request.
<br/>

```rust
deleteLinkRequest(request_id: u64)
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = parent or caller = child or caller = owner.

>[!NOTE]
>If a parent changes its mind about creating a link, and the link request has not yet been approved or denied, it can delete the request.
<br/>

```rust
acceptLink(request_id: u64)
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = child or caller = owner.

>[!NOTE]
>The link between the parent and the child is created and if `keys` was specified in the request, they are initialized.
<br/>

```rust
requestUnlink(
    parent_id: u64,
    child_id: u64,
    reason: ManagedBuffer,
) -> u64
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = child.address.

>[!NOTE]
>Creates an unlink request between identities `parent_id` and `child_id` for the specified `reason`. Returns the ID of the newly created unlink request.
<br/>

```rust
deleteUnlinkRequest(request_id: u64)
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = parent or caller = child or caller = owner.

>[!NOTE]
>If a child changes its mind about removing a link, and the unlink request has not yet been approved or denied, it can delete the request.
<br/>

```rust
acceptUnlink(request_id: u64)
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = parent or caller = owner.

>[!NOTE]
>The link between the parent and the child is removed.
<br/>

```rust
removeIdentityLink(parent_id: u64, child_id: u64)
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = parent or caller = owner.

>[!NOTE]
>A parent can remove a link to a child at any point in time.
<br/>

```rust
addModifier(
    identity_id: u64,
    key: &ManagedBuffer,
    opt_modifier: OptionalValue<ManagedAddress>,
)
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = parent or caller = owner, link must exist between caller (parent) and identity_id (child), key must not hold any value.

>[!NOTE]
>Empowers the caller (or `opt_modifier` if specified) to write under the child's (identity_id) `key`.
<br/>

```rust
removeModifier(
    identity_id: u64,
    key: &ManagedBuffer,
    opt_modifier: OptionalValue<ManagedAddress>,
)
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = existing modifier or caller = owner.

>[!NOTE]
>Removes access of the caller (or `opt_modifier` if specified) to write under the child's (identity_id) `key`.
<br/>

```rust
addIdentityKeyValue(
    identity_id: u64,
    key: ManagedBuffer,
    value: ManagedBuffer,
)
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = modifier or caller = owner.

>[!NOTE]
>Adds the `value` to the `key` in the child's (identity_id) storage.
<br/>

```rust
addIdentityKeysValues(
    identity_id: u64,
    keys_values: MultiValueEncoded<(ManagedBuffer, ManagedBuffer)>,
)
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = modifier or caller = owner.

>[!NOTE]
>Adds multiple key-value pairs in the child's (identity_id) storage.
<br/>

```rust
editIdentityKeyValue(
    identity_id: u64,
    key: ManagedBuffer,
    value_id: u64,
    new_value: ManagedBuffer,
)
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = initial modifier or caller = owner.

>[!NOTE]
>Replaces the specified `value_id`'s value with the `new_value`.
<br/>

```rust
removeIdentityKeyValue(
    identity_id: u64,
    key: ManagedBuffer,
    value_id: u64,
)
```
>[!IMPORTANT]
>*Requirements:* state = active, caller = modifier or caller = owner. If there are no modifiers for the specified key, the identity's address can also call this endpoint.

>[!NOTE]
>Removes the specified `value_id` from the identity's (identity_id) storage.
<b/>

```rust
setStateActive()
```
>[!IMPORTANT]
*Requirements:* the caller must be the SC owner.

>[!NOTE]
>Sets the SC state as active.
<br/>

```rust
setStateInactive()
```
>[!IMPORTANT]
*Requirements:* the caller must be the SC owner.

>[!NOTE]
>Sets the SC state as inactive.

<br/>

## View functions

<br/>

```rust
getState() -> State
```
>Returns the state of the SC (Active or Inactive).
<br/>

```rust
getIdentity(identity_id: u64) -> Identity
```
>Returns the Identity object corresponding to the `identity_id` parameter.
<br/>

```rust
getLastIdentityId() -> u64
```
>Returns the `ID - 1` of the last registered identity.
<br/>

```rust
getIdentityKeyModifiers(identity_id: u64, key: &ManagedBuffer) -> UnorderedSetMapper<ManagedAddress>
```
>Returns the list of allowed modifier addresses under the `key` of the specified `identity_id`.
<br/>

```rust
getIdentityKeys(identity_id: u64) -> ManagedVec<ManagedBuffer>
```
>Returns the list of the storage keys of the specified `identity_id`.
<br/>

```rust
getLastIdentityKeyId(identity_id: u64, key: &ManagedBuffer) -> u64
```
>Returns the `ID - 1` of the last registered value under the `key` of the specified `identity_id`.
<br/>

```rust
getIdentityKeyValue(identity_id: u64, key: &ManagedBuffer, value_id: u64) -> Value
```
>Returns the Value object corresponding to the `identity_id`, `key` and `value_id` parameters.
<br/>

```rust
getIdentityLink(link_id: u64) -> IdentityLink
```
>Returns the IdentityLink object corresponding to the `link_id` parameter.
<br/>

```rust
getLastIdentityLinkId() -> u64
```
>Returns the `ID - 1` of the last registered identity link.
<br/>

```rust
getChildrenLinks(child_id: u64) -> UnorderedSetMapper<u64>
```
>Returns the list of IDs of the links in which `child_id` is present.
<br/>

```rust
getParentLinks(parent_id: u64) -> UnorderedSetMapper<u64>
```
>Returns the list of IDs of the links in which `parent_id` is present.
<br/>

```rust
getLinkRequest(request_id: u64) -> LinkRequest
```
>Returns the LinkRequest object corresponding to the `request_id` parameter.
<br/>

```rust
getLastLinkRequestId() -> u64
```
>Returns the `ID - 1` of the last registered link request.
<br/>

```rust
getUnlinkRequest(request_id: u64) -> UnlinkRequest
```
>Returns the UnlinkRequest object corresponding to the `request_id` parameter.
<br/>

```rust
getLastUnlinkRequestId() -> u64
```
>Returns the `ID - 1` of the last registered unlink request.
<br/>

```rust
getIdentityByAddress(wallet: &ManagedAddress) -> Option<Identity>
```
>Returns Some(identity) if an identity with the specified wallet address is found and None otherwise.
<br/>

```rust
getIdentityByLegalId(legal_id: BigUint) -> Option<Identity>
```
>Returns Some(identity) if an identity with the specified legal_id is found and None otherwise.
<br/>

```rust
getIdentityKeyValues(identity_id: u64, key: &ManagedBuffer) -> ManagedVec<Value>
```
>Returns all the values under the specified `key` of `identity_id`.
<br/>

```rust
getChildrenIdentitiesOfParent(parent_id: u64) -> ManagedVec<Identity>
```
>Returns all the identities that are linked to `parent_id` as children.
<br/>

```rust
getParentIdentitiesOfChild(child_id: u64) -> ManagedVec<Identity>
```
>Returns all the identities that are linked to `child_id` as parents.
<br/>

```rust
getLinkByIds(parent_id: u64, child_id: u64) -> Option<IdentityLink>
```
>Returns Some(identity_link) if a link is found between the specified `parent_id` and `child_id` and None otherwise.
<br/>

```rust
getLinkedIdentities(identity_id: u64) -> MultiValueEncoded<(Identity, IdentityLink)>
```
>Returns a list containing, for each link of `identity_id`, the identity object (parent or child) and the identity link object.
<br/>

```rust
isParentOfChild(parent_id: u64, child_id: u64) -> bool
```
>Returns true if `parent_id` has a link with `child_id` as parent and false otherwise.
<br/>

```rust
isChildOfParent(child_id: u64, parent_id: u64) -> bool
```
>Returns true if `child_id` has a link with `parent_id` as child and false otherwise.
<br/>

```rust
getChildrenWithSameLastValue(
    parent_id: u64,
    key: ManagedBuffer,
    last_value: ManagedBuffer,
    opt_relation: OptionalValue<ManagedBuffer>,
) -> ManagedVec<Identity>
```
>Returns a list of children identities of the specified `parent_id` that, for the specified `key`, have the same `last_value`.
>Example: useful for a school that has employees as children and a `job` key, for selecting all the teachers.
>We look only at the last value because, if job is changed, you may want to add the new job instead of editing the old one, for the purpose of keeping a historical record of all the employee's jobs.
<br/>

```rust
getMultipleIdentities(identity_ids: ManagedVec<u64>) -> ManagedVec<Identity>
```
>Returns a list of all identity objects corresponding to the IDs in the `identity_ids` list.
<br/>

```rust
getLastValueOfKey(identity_id: u64, key: &ManagedBuffer) -> Option<Value>
```
>Returns Some(value) for the last value object recorded under the specified `key` in `identity_id`. If no value exists under key, None is returned.
<br/>

```rust
getLinkRequestsByParent(parent_id: u64) -> ManagedVec<LinkRequest>
```
>Returns a list with all the link requests initiated by `parent_id`.
<br/>

```rust
getLinkRequestsByChild(child_id: u64) -> ManagedVec<LinkRequest>
```
>Returns a list with all the link requests received by `child_id`.
<br/>

```rust
getUnlinkRequestsByParent(parent_id: u64) -> ManagedVec<UnlinkRequest>
```
>Returns a list with all the unlink requests received by `parent_id`.
<br/>

```rust
getUnlinkRequestsByChild(child_id: u64) -> ManagedVec<UnlinkRequest>
```
>Returns a list with all the unlink requests initiated by `child_id`.

<br/>

## Custom types

<br/>

```rust
pub enum State {
    Inactive,
    Active,
}
```

<br/>

```rust
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
```

<br/>

```rust
pub struct Value<M: ManagedTypeApi> {
    pub id: u64,
    pub value: ManagedBuffer<M>,
    pub modifier: ManagedAddress<M>,
    pub timestamp: u64,
}
```

<br/>

```rust
pub struct IdentityLink<M: ManagedTypeApi> {
    pub id: u64,
    pub child_id: u64,
    pub parent_id: u64,
    pub relation: ManagedBuffer<M>,
}
```

<br/>

```rust
pub struct LinkRequest<M: ManagedTypeApi> {
    pub id: u64,
    pub parent_id: u64,
    pub child_id: u64,
    pub relation: ManagedBuffer<M>,
    pub keys: ManagedVec<M, ManagedBuffer<M>>,
}
```

<br/>

```rust
pub struct UnlinkRequest<M: ManagedTypeApi> {
    pub id: u64,
    pub parent_id: u64,
    pub child_id: u64,
    pub reason: ManagedBuffer<M>,
}
```
