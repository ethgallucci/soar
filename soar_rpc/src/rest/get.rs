#![allow(dead_code)]

pub struct GetErr {}

pub(crate) mod bank {
    #[derive(Debug)]
    pub enum Bank {
        Balances { address: String },
        BalancesDenom { address: String, denom: String },
        DenomOwners { denom: String },
        DenomsMetadata { denom: String },
        Params,
        Supply,
        SupplyDenom { denom: String },
    }

    impl TryInto<Bank> for String {
        type Error = super::GetErr;

        fn try_into(self) -> Result<Bank, Self::Error> {
            match self.clone().as_str() {
                "--params" => Ok(Bank::Params),
                "--supply" => Ok(Bank::Supply),
                _ => Err(Self::Error {}),
            }
        }
    }
}

pub(crate) mod distrib {
    #[derive(Debug)]
    pub enum Distribution {
        CommunityPool,
        DelegationTotalRewards {
            delegator_addr: String,
        },
        DelegationRewards {
            delegator_addr: String,
            validator_addr: String,
        },
        DelegatorValidators {
            delegator_addr: String,
        },
        DelegatorWithdrawAddress {
            delegator_addr: String,
        },
        Params,
        ValidatorCommission {
            validator_addr: String,
        },
        ValidatorOutstandingRewards {
            validator_addr: String,
        },
        ValidatorSlashes {
            validator_addr: String,
        },
    }
}

pub(crate) mod gov {
    #[derive(Debug)]
    pub enum Governance {
        Params {
            param_type: String,
        },
        Proposals,
        /// param: proposal_id
        ProposalsAddr(String),
        /// param: proposal_id
        ProposalsDeposits(String),
        ProposalDepositors {
            proposal_id: String,
            depositor: String,
        },
        /// param: proposal_id
        ProposalTally(String),
        /// param: proposal_id
        ProposalVotes(String),
        ProposalVoter {
            proposal_id: String,
            voter: String,
        },
    }
}

pub(crate) mod mint {
    #[derive(Debug)]
    pub enum Mint {
        AnnualProvisions,
        Inflation,
        Params,
    }
}
