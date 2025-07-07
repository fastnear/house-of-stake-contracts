use crate::*;
use near_sdk::require;

#[derive(Clone)]
#[near(serializers=[json, borsh])]
pub enum VenearGrowthConfig {
    FixedRate(Box<VenearGrowthConfigFixedRate>),
}

/// The fixed annual growth rate of veNEAR tokens.
/// Note, the growth rate can be changed in the future through the upgrade mechanism, by introducing
/// timepoints when the growth rate changes.
#[derive(Clone)]
#[near(serializers=[json, borsh])]
pub struct VenearGrowthConfigFixedRate {
    /// The growth rate of veNEAR tokens per nanosecond. E.g. `6 / (100 * NUM_SEC_IN_YEAR * 10**9)`
    /// means 6% annual growth rate.
    /// Note, the denominator has to be `10**30` to avoid precision issues.
    pub annual_growth_rate_ns: Fraction,
}

impl From<VenearGrowthConfigFixedRate> for VenearGrowthConfig {
    fn from(config: VenearGrowthConfigFixedRate) -> Self {
        Self::FixedRate(Box::new(config))
    }
}

impl VenearGrowthConfig {
    pub fn calculate(
        &self,
        previous_timestamp: TimestampNs,
        current_timestamp: TimestampNs,
        balance: NearToken,
    ) -> NearToken {
        require!(
            current_timestamp >= previous_timestamp,
            "Timestamp must be increasing"
        );
        require!(
            current_timestamp == truncate_to_seconds(current_timestamp),
            "Current timestamp must be truncated to seconds"
        );
        require!(
            previous_timestamp == truncate_to_seconds(previous_timestamp),
            "Previous timestamp must be truncated to seconds"
        );
        if previous_timestamp == current_timestamp {
            return NearToken::from_yoctonear(0);
        }
        let truncated_near_balance = truncate_near_to_millis(balance);
        match self {
            VenearGrowthConfig::FixedRate(config) => {
                let growth_period_ns = current_timestamp.0 - previous_timestamp.0;
                NearToken::from_yoctonear(
                    config
                        .annual_growth_rate_ns
                        .u384_mul(growth_period_ns as _, truncated_near_balance.as_yoctonear()),
                )
            }
        }
    }
}
