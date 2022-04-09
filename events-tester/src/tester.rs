#![no_std]
pub mod events;
pub mod proxys;

elrond_wasm::imports!();
use proxys::staking_provider_delegation_proxy::ProxyTrait as DelegationProxy;

#[elrond_wasm::derive::contract]
pub trait Tester: events::EventsModule {
    #[view(getDelegationContract)]
    #[storage_mapper("delegation_contract")]
    fn delegation_contract(&self) -> SingleValueMapper<ManagedAddress>;

    /// Staking provider delegation contract proxy
    #[proxy]
    fn delegation_proxy(
        &self,
        sc_address: ManagedAddress,
    ) -> proxys::staking_provider_delegation_proxy::Proxy<Self::Api>;

    #[init]
    fn init(&self, delegation_contract_address: ManagedAddress) {
        self.delegation_contract().set(delegation_contract_address);
    }

    #[endpoint]
    fn test_event(&self) {
        let caller = self.blockchain().get_caller();
        let payment_amount = BigUint::from(1000u32);
        self.emit_deposit_event(&caller, &payment_amount);
    }

    #[endpoint]
    fn test_double_event(&self) {
        let caller = self.blockchain().get_caller();
        let payment_amount = BigUint::from(1000u32);
        self.emit_deposit_event(&caller, &payment_amount);
        self.emit_mint_event(&payment_amount, &caller);
    }

    #[endpoint]
    fn test_event_in_callback(&self) {
        let caller = self.blockchain().get_caller();
        let payment_amount = BigUint::from(1000u32);
        // Do the asynchronous transaction to the staking provider
        self.delegation_proxy(self.delegation_contract().get())
            .delegate_endpoint(payment_amount.clone())
            .with_egld_transfer(payment_amount.clone())
            .async_call()
            .with_callback(self.callbacks().callback_simple(&caller, &payment_amount))
            .call_and_exit();
    }

    #[callback]
    fn callback_simple(
        &self,
        caller: &ManagedAddress,
        payment_amount: &BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(_) => {
                self.emit_deposit_event(caller, payment_amount);
            }
            ManagedAsyncCallResult::Err(_err) => {
                // nothing
            }
        }
    }

    #[endpoint]
    fn test_double_event_in_callback(&self) {
        let caller = self.blockchain().get_caller();
        let payment_amount = BigUint::from(1000u32);
        // Do the asynchronous transaction to the staking provider
        self.delegation_proxy(self.delegation_contract().get())
            .delegate_endpoint(payment_amount.clone())
            .with_egld_transfer(payment_amount.clone())
            .async_call()
            .with_callback(self.callbacks().callback_double(&caller, &payment_amount))
            .call_and_exit();
    }

    #[callback]
    fn callback_double(
        &self,
        caller: &ManagedAddress,
        payment_amount: &BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(_) => {
                self.emit_deposit_event(caller, payment_amount);
                self.emit_mint_event(payment_amount, caller);
            }
            ManagedAsyncCallResult::Err(_err) => {
                // nothing
            }
        }
    }
}
