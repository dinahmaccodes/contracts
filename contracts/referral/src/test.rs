#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Env, Address, String, Symbol, BytesN, Vec};
use crate::contract::{ReferralContract, ReferralContractClient};
use crate::types::{Referral, ReferralStatus, Error};

// Basic scaffolding, tests to follow.
