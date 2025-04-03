#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait TFNDAOContract<ContractReader> {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
