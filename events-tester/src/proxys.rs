pub mod staking_provider_delegation_proxy {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait StakingProviderDelegation {
        #[payable("EGLD")]
        #[endpoint(delegate)]
        fn delegate_endpoint(&self, #[payment] payment: BigUint);

        #[endpoint(claimRewards)]
        fn claim_rewards(&self);

        #[endpoint(reDelegateRewards)]
        fn redelegate_rewards(&self);

        // starts unbond period
        #[endpoint(unDelegate)]
        fn undelegate_endpoint(&self, amount: BigUint);

        #[endpoint(withdraw)]
        fn withdraw_endpoint(&self);
    }
}
