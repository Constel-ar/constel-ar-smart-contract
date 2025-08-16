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

    MilestoneNotFound = 8,
    InvalidMilestoneAmount = 9,
    InvalidGoalAmount = 10,
    InvalidMinDonation = 11,
    CampaignAlreadyExists = 12,
    MilestoneAmountNotIncreasing = 13,

    AmountMustBePositive = 14,
    ContributionBelowMinimum = 15,
    CampaignGoalExceeded = 16,
}
