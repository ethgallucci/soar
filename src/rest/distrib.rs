#[derive(Debug)]
pub enum DistributionQuery {
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
    ValidatorSlashes { validator_addr: String, },
}
