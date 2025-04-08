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

<br/>

## View functions

<br/>

```rust
getState() -> State
```
>Returns the state of the SC (Active or Inactive).

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
