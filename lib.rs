#![cfg_attr(not(feature = "std"), no_std, no_main)]

        
#[openbrush::implementation(PSP22, PSP22Ownable,PSP22Burnable,PSP22Mintable,PSP22Pausable)]
#[openbrush::contract]
pub mod my_psp22 {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct RobinCoin {
    	#[storage_field]
		psp22: psp22::Data,
		#[storage_field]
		ownable: ownable::Data,
		#[storage_field]
		pausable: pausable::Data,
    }
    
    impl RobinCoin {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let mut _instance = Self::default();
			psp22::Internal::_mint(&mut _instance, Self::env().caller(), initial_supply).expect("Should mint"); 
			ownable::Internal::_init_with_owner(&mut _instance Self::env().caller());
			_instance
        }

		#[ink(message)]
		#[openbrush::modifiers(only_owner)]
		pub fn change_state(&mut self) -> Result<(), PSP22Error> {
			if self.paused() {
                self._unpause()
            } else {
                self._pause()
            }
		}
    }
}