use evmil::evm;
use evmil::evm::AbstractStack;
use evmil::util::{w256, Interval, Top};

type Word = Interval<w256>;

const ZERO: Word = Interval::new(w256::ZERO, w256::ZERO);
const ONE: Word = Interval::new(w256::ONE, w256::ONE);
const TWO: Word = Interval::new(w256::TWO, w256::TWO);
const ONETWO: Word = Interval::new(w256::ONE, w256::TWO);
const THREE: Word = Interval::new(w256::THREE, w256::THREE);
const UNKNOWN: Word = Interval::TOP;

#[test]
fn test_abstract_stack_01() {
    let mut st = AbstractStack::new(0, vec![]);
    assert_eq!(st.values(), vec![]);
    assert_eq!(st.push(UNKNOWN), &AbstractStack::new(1, vec![]));
}

#[test]
fn test_abstract_stack_02() {
    let mut st = AbstractStack::new(0, vec![]);
    assert_eq!(
        st.push(UNKNOWN).push(UNKNOWN),
        &AbstractStack::new(2, vec![])
    );
}

#[test]
fn test_abstract_stack_03() {
    let mut st = AbstractStack::new(0, vec![]);
    assert_eq!(
        st.push(UNKNOWN).push(UNKNOWN).pop(),
        &AbstractStack::new(1, vec![])
    );
}

#[test]
fn test_abstract_stack_04() {
    let mut st = AbstractStack::new(0, vec![]);
    assert_eq!(
        st.push(UNKNOWN).push(ZERO),
        &AbstractStack::new(1, vec![ZERO])
    );
}

#[test]
fn test_abstract_stack_05() {
    let mut st = AbstractStack::new(0, vec![]);
    assert_eq!(
        st.push(UNKNOWN).push(ZERO).push(ONE),
        &AbstractStack::new(1, vec![ZERO, ONE])
    );
}

#[test]
fn test_abstract_stack_06() {
    let mut st = AbstractStack::new(0, vec![]);
    assert_eq!(
        st.push(UNKNOWN).push(ZERO).push(ONE).pop(),
        &AbstractStack::new(1, vec![ZERO])
    );
}

#[test]
fn test_abstract_stack_07() {
    let mut st = AbstractStack::new(0, vec![]);
    assert_eq!(
        st.push(UNKNOWN).push(ZERO).push(ONE).pop().pop(),
        &AbstractStack::new(1, vec![])
    );
}

#[test]
fn test_abstract_stack_08() {
    let mut st = AbstractStack::new(0, vec![]);
    assert_eq!(
        st.push(UNKNOWN).push(ZERO).push(ONE).pop().pop().pop(),
        &AbstractStack::empty()
    );
}

#[test]
fn test_abstract_stack_09() {
    let st = AbstractStack::new(0, vec![ONE, TWO, THREE]);
    assert_eq!(st.values(), vec![ONE, TWO, THREE]);
    assert_eq!(st.set(0, ZERO), AbstractStack::new(0, vec![ONE, TWO, ZERO]));
}

#[test]
fn test_abstract_stack_0a() {
    let st = AbstractStack::new(0, vec![ONE, TWO, THREE]);
    assert_eq!(
        st.set(1, ZERO),
        AbstractStack::new(0, vec![ONE, ZERO, THREE])
    );
}

#[test]
fn test_abstract_stack_0b() {
    let st = AbstractStack::new(0, vec![ONE, TWO, THREE]);
    assert_eq!(st.set(2, UNKNOWN), AbstractStack::new(1, vec![TWO, THREE]));
}

#[test]
fn test_abstract_stack_0c() {
    let st = AbstractStack::new(0, vec![ONE, UNKNOWN, THREE]);
    assert_eq!(st.values(), vec![ONE, UNKNOWN, THREE]);
    assert_eq!(st.set(2, UNKNOWN), AbstractStack::new(2, vec![THREE]));
}

#[test]
fn test_abstract_stack_0d() {
    let st = AbstractStack::new(1, vec![]);
    assert_eq!(st.set(0, ONE), AbstractStack::new(0, vec![ONE]));
}

#[test]
fn test_abstract_stack_0e() {
    let st = AbstractStack::new(1, vec![TWO]);
    assert_eq!(st.set(1, ONE), AbstractStack::new(0, vec![ONE, TWO]));
}

#[test]
fn test_abstract_stack_0f() {
    let st = AbstractStack::new(2, vec![TWO]);
    assert_eq!(
        st.set(2, ONE),
        AbstractStack::new(0, vec![ONE, UNKNOWN, TWO])
    );
}

#[test]
fn test_abstract_stack_0g() {
    let st = AbstractStack::new(2..=3, vec![TWO]);
    assert_eq!(
        st.set(2, ONE),
        AbstractStack::new(0..=1, vec![ONE, UNKNOWN, TWO])
    );
}

#[test]
fn test_abstract_stack_10() {
    let st1 = AbstractStack::new(1, vec![ONE]);
    let st2 = AbstractStack::new(0, vec![ONE]);
    assert_eq!(st1.join(&st2), AbstractStack::new(0..=1, vec![ONE]));
}
#[test]
fn test_abstract_stack_11() {
    let st1 = AbstractStack::new(0, vec![ONE]);
    let st2 = AbstractStack::new(1, vec![ONE]);
    assert_eq!(st1.join(&st2), AbstractStack::new(0..=1, vec![ONE]));
}
#[test]
fn test_abstract_stack_12() {
    let st1 = AbstractStack::new(0, vec![TWO, ONE]);
    let st2 = AbstractStack::new(1, vec![ONE]);
    assert_eq!(st1.join(&st2), AbstractStack::new(1, vec![ONE]));
}
#[test]
fn test_abstract_stack_13() {
    let st1 = AbstractStack::new(1, vec![ONE]);
    let st2 = AbstractStack::new(0, vec![TWO, ONE]);
    assert_eq!(st1.join(&st2), AbstractStack::new(1, vec![ONE]));
}
#[test]
fn test_abstract_stack_14() {
    let st1 = AbstractStack::new(0, vec![ONE, ONE]);
    let st2 = AbstractStack::new(0, vec![TWO, ONE]);
    assert_eq!(st1.join(&st2), AbstractStack::new(0, vec![ONETWO, ONE]));
}
#[test]
fn test_abstract_stack_15() {
    let st1 = AbstractStack::new(1, vec![ONE, ONE]);
    let st2 = AbstractStack::new(0, vec![TWO, ONE]);
    assert_eq!(st1.join(&st2), AbstractStack::new(0..=1, vec![ONETWO, ONE]));
}
#[test]
fn test_abstract_stack_16() {
    let st1 = AbstractStack::new(0..=1, vec![ONE, ONE]);
    let st2 = AbstractStack::new(0, vec![TWO, ONE]);
    assert_eq!(st1.join(&st2), AbstractStack::new(0..=1, vec![ONETWO, ONE]));
}
#[test]
fn test_abstract_stack_17() {
    let st1 = AbstractStack::new(0, vec![ONE, ONE]);
    let st2 = AbstractStack::new(0..=1, vec![TWO, ONE]);
    assert_eq!(st1.join(&st2), AbstractStack::new(0..=1, vec![ONETWO, ONE]));
}

// Tests for set()
// Force upper expansion
// Force rebalance
