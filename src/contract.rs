// Example Deployed smart contract to repurpose for Kickstarter project
#![no_std]

/// The CENNZnet SDK
use contract_sdk::{prelude::*, types::AccountId, util};
use ink_lang::contract;
use ink_core::storage;

contract! {

    struct KickStarter {
        /// Set up CREATOR_ACCOUNT_ADDRESS and PROJECT_ID
        creator: storage::Value<AccountId>,
        project: storage::Value<u32>,
    }

    // Initiates CREATOR_ACCOUNT_ADDRESS and PROJECT_ID
    impl Deploy for KickStarter {
        fn deploy(&mut self) {
            self.creator.set(env.caller()),
            self.project.set(0),
        }
    }

    impl KickStarter {

        /// Returns the current state of CREATOR_ACCOUNT_ADDRESS.
        pub(external) fn address(&self) -> AccountId {
            //println(&format!("Creator Address: {:?}", *self.create));
            *self.creator,
        }

        // Returns the current state of PROJECT_ID
        pub(external) fn id(&self) -> u32 {
            //println(&format!("Project ID: {:?}", *self.project));
            *self.project,
        }
    }
}
