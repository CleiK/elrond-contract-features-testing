#![no_std]
#![allow(clippy::type_complexity)]

elrond_wasm::imports!();

#[elrond_wasm::derive::contract]
pub trait DelegationMock {
    #[init]
    fn init(&self) {
        sc_print!("delegation-mock | init{}", "");
    }

    #[payable("EGLD")]
    #[endpoint(delegate)]
    fn delegate_endpoint(&self, #[payment] _payment: BigUint) {
        sc_print!("delegation-mock | delegate{}", "");
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        sc_print!("delegation-mock | claim_rewards{}", "");
    }

    #[endpoint(reDelegateRewards)]
    fn redelegate_rewards(&self) {
        sc_print!("delegation-mock | redelegate_rewards{}", "");
    }

    // starts unbond period
    #[endpoint(unDelegate)]
    fn undelegate_endpoint(&self, _amount: BigUint) {
        sc_print!("delegation-mock | undelegate_endpoint{}", "");
    }

    #[endpoint(withdraw)]
    fn withdraw_endpoint(&self) {
        sc_print!("delegation-mock | withdraw_endpoint{}", "");
    }
}
