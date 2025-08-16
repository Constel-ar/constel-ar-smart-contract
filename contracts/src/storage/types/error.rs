use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    ContractInitialized = 0,
    ContractNotInitialized = 1,
    NonExistentUser = 2,
    MathOverflow = 3,
    MathUnderflow = 4,
    NotImplemented = 5,
    CampaignNotFound = 6,
    ProofNotFound = 7,
    AmountMustBePositive = 8,
    CampaignNotRunning = 9,
    ContributionBelowMinimum = 10,
    CampaignGoalExceeded = 11,
    CampaignNorRefundable = 12,
    ContributionNotFound = 13,
    InvalidCampaignId = 14,
    CampaignAlreadyExists = 15,
    InsufficientFunds = 16,
    InvalidInput = 17
}
