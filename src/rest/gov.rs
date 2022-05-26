#[derive(Debug)]
pub enum GovQuery {
    Params(String),
    Proposals,
    Proposals(String),
    ProposalDeposits(String),
    ProposalDepositories { id: String, depositor: String },
    ProposalTally(String),
    ProposalVotes(String),
    ProposalVoter { id: String, voter: String },
}
