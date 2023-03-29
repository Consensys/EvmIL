use std::cmp::Ordering;

// ===================================================================
// EIPS
// ===================================================================

pub const EIP_1559 : EIP = EIP("Fee market change for ETH 1.0 chain");
pub const EIP_2565 : EIP = EIP("ModExp Gas Cost");
pub const EIP_2929 : EIP = EIP("Gas cost increases for state access opcodes");
pub const EIP_2718 : EIP = EIP("Typed Transaction Envelope");
pub const EIP_2930 : EIP = EIP("Optional access lists");
pub const EIP_3198 : EIP = EIP("BASEFEE opcode");
pub const EIP_3529 : EIP = EIP("Reduction in refunds");
pub const EIP_3541 : EIP = EIP("Reject new contract code starting with the 0xEF byte");
pub const EIP_3554 : EIP = EIP("Difficulty Bomb Delay to December 2021");
pub const EIP_3651 : EIP = EIP("Warm COINBASE");
pub const EIP_3675 : EIP = EIP("Upgrade consensus to Proof-of-Stake");
pub const EIP_3855 : EIP = EIP("PUSH0 instruction");
pub const EIP_3860 : EIP = EIP("Limit and meter initcode");
pub const EIP_4345 : EIP = EIP("Difficulty Bomb Delay to June 2022");
pub const EIP_4399 : EIP = EIP("Supplant DIFFICULTY opcode with PREVRANDAO");
pub const EIP_4895 : EIP = EIP("Beacon chain push withdrawals as operations");
pub const EIP_5133 : EIP = EIP("Delaying Difficulty Bomb to mid-September 2022");

// ===================================================================
// Forks
// ===================================================================

pub const HOMESTEAD : Fork = Fork{id:2016_03_14, eips: &[]};
pub const TANGERINE_WHISTLE : Fork = Fork{id:2016_10_18, eips: &[]};
pub const SUPRIOUS_DRAGON : Fork = Fork{id:2016_11_22, eips: &[]};
pub const BYZANTIUM : Fork = Fork{id:2017_10_16, eips: &[]};
pub const CONSTANTINOPLE_PETERSBURG : Fork = Fork{id:2019_02_28, eips: &[]};
pub const INSTANBUL : Fork = Fork{id:2019_12_07, eips: &[]};
pub const MUIR_GLACIER : Fork = Fork{id:2020_01_02, eips: &[]};

pub const BERLIN : Fork = Fork{id:2021_04_15, eips: &[EIP_2565,EIP_2929,EIP_2718,EIP_2930]};
pub const LONDON : Fork = Fork{id:2021_08_05, eips: &[EIP_1559,EIP_3198,EIP_3529,EIP_3541,EIP_3554]};
pub const ARROW_GLACIER : Fork = Fork{id:2021_12_09, eips: &[EIP_4345]};
pub const GRAY_GLACIER : Fork = Fork{id:2022_06_30, eips: &[EIP_5133]};
pub const PARIS : Fork = Fork{id:2022_09_15, eips: &[EIP_3675,EIP_4399]};
pub const SHANGHAI : Fork = Fork{id:2023_04_12, eips: &[EIP_3651,EIP_3855,EIP_3860,EIP_4895]};

// ===================================================================
// EIP
// ===================================================================

#[derive(Debug,Eq,PartialEq)]
pub struct EIP(&'static str);

// ===================================================================
// Fork Definition
// ===================================================================

#[derive(Debug,Eq,PartialEq)]
pub struct Fork {
    /// Fork identifier which uniquely determines the fork based on
    /// its activation date.
    id: usize,
    /// List of EIPs activated by this fork.
    eips: &'static [EIP]
}

impl PartialOrd for Fork {
    fn partial_cmp(&self, other: &Fork) -> Option<Ordering> {
        if self.id < other.id {
            Some(Ordering::Less)
        } else if self.id > other.id {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for Fork {
    fn cmp(&self, other: &Fork) -> Ordering {
        // Safe because we know `partiual_cmp` never returns `None`.
        self.partial_cmp(other).unwrap()
    }
}
