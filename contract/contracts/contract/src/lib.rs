#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct Campaign {
    pub reward_per_kg: i128,
    pub total_funds: i128,
}

const CAMPAIGN: Symbol = symbol_short!("CAMPAIGN");

#[contract]
pub struct BeachCleanupBounty;

#[contractimpl]
impl BeachCleanupBounty {
    pub fn init_campaign(env: Env, reward_per_kg: i128, total_funds: i128) {
        let campaign = Campaign {
            reward_per_kg,
            total_funds,
        };

        env.storage().instance().set(&CAMPAIGN, &campaign);
    }

    pub fn submit_cleanup(env: Env, volunteer: Address, kilograms: i128) -> i128 {
        let mut campaign: Campaign = env.storage().instance().get(&CAMPAIGN).unwrap();

        let reward = kilograms * campaign.reward_per_kg;

        if reward > campaign.total_funds {
            panic!("Insufficient funds");
        }

        campaign.total_funds -= reward;
        env.storage().instance().set(&CAMPAIGN, &campaign);

        env.storage().persistent().set(&volunteer, &reward);

        reward
    }

    pub fn get_reward(env: Env, volunteer: Address) -> i128 {
        env.storage().persistent().get(&volunteer).unwrap_or(0)
    }

    pub fn get_campaign(env: Env) -> Campaign {
        env.storage().instance().get(&CAMPAIGN).unwrap()
    }
}