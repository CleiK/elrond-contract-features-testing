elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::derive::module]
pub trait EventsModule {
    fn emit_deposit_event(&self, caller: &ManagedAddress, payment_amount: &BigUint) {
        let epoch = self.blockchain().get_block_epoch();
        self.deposit_event(payment_amount, caller, epoch)
    }

    fn emit_mint_event(&self, amount: &BigUint, to: &ManagedAddress) {
        let epoch = self.blockchain().get_block_epoch();
        self.mint_event(amount, to, epoch)
    }

    #[event("deposit")]
    fn deposit_event(
        &self,
        #[indexed] payment_amount: &BigUint,
        #[indexed] caller: &ManagedAddress,
        #[indexed] epoch: u64,
    );

    #[event("mint")]
    fn mint_event(
        &self,
        #[indexed] amount: &BigUint,
        #[indexed] to: &ManagedAddress,
        #[indexed] epoch: u64,
    );
}
