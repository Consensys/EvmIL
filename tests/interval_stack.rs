use evmil::evm;
use evmil::util::{w256, Interval, IntervalStack, Top};

type Word = Interval<w256>;

const ZERO: Word = Interval::new(w256::ZERO, w256::ZERO);
const ONE: Word = Interval::new(w256::ONE, w256::ONE);
const TWO: Word = Interval::new(w256::TWO, w256::TWO);
const ONETWO: Word = Interval::new(w256::ONE, w256::TWO);
const THREE: Word = Interval::new(w256::THREE, w256::THREE);
const UNKNOWN: Word = Interval::TOP;

#[test]
fn test_abstract_stack_01() {
    let mut st = IntervalStack::new(0, vec![]);
    assert_eq!(st.values(), vec![]);
    assert_eq!(st.push(UNKNOWN), &IntervalStack::new(1, vec![]));
}

#[test]
fn test_abstract_stack_02() {
    let mut st = IntervalStack::new(0, vec![]);
    assert_eq!(
        st.push(UNKNOWN).push(UNKNOWN),
        &IntervalStack::new(2, vec![])
    );
}

#[test]
fn test_abstract_stack_03() {
    let mut st = IntervalStack::new(0, vec![]);
    assert_eq!(
        st.push(UNKNOWN).push(UNKNOWN).pop(),
        &IntervalStack::new(1, vec![])
    );
}

#[test]
fn test_abstract_stack_04() {
    let mut st = IntervalStack::new(0, vec![]);
    assert_eq!(
        st.push(UNKNOWN).push(ZERO),
        &IntervalStack::new(1, vec![ZERO])
    );
}

#[test]
fn test_abstract_stack_05() {
    let mut st = IntervalStack::new(0, vec![]);
    assert_eq!(
        st.push(UNKNOWN).push(ZERO).push(ONE),
        &IntervalStack::new(1, vec![ZERO, ONE])
    );
}

#[test]
fn test_abstract_stack_06() {
    let mut st = IntervalStack::new(0, vec![]);
    assert_eq!(
        st.push(UNKNOWN).push(ZERO).push(ONE).pop(),
        &IntervalStack::new(1, vec![ZERO])
    );
}

#[test]
fn test_abstract_stack_07() {
    let mut st = IntervalStack::new(0, vec![]);
    assert_eq!(
        st.push(UNKNOWN).push(ZERO).push(ONE).pop().pop(),
        &IntervalStack::new(1, vec![])
    );
}

#[test]
fn test_abstract_stack_08() {
    let mut st = IntervalStack::new(0, vec![]);
    assert_eq!(
        st.push(UNKNOWN).push(ZERO).push(ONE).pop().pop().pop(),
        &IntervalStack::empty()
    );
}

#[test]
fn test_abstract_stack_09() {
    let st = IntervalStack::new(0, vec![ONE, TWO, THREE]);
    assert_eq!(st.values(), vec![ONE, TWO, THREE]);
    assert_eq!(st.set(0, ZERO), IntervalStack::new(0, vec![ONE, TWO, ZERO]));
}

#[test]
fn test_abstract_stack_0a() {
    let st = IntervalStack::new(0, vec![ONE, TWO, THREE]);
    assert_eq!(
        st.set(1, ZERO),
        IntervalStack::new(0, vec![ONE, ZERO, THREE])
    );
}

#[test]
fn test_abstract_stack_0b() {
    let st = IntervalStack::new(0, vec![ONE, TWO, THREE]);
    assert_eq!(st.set(2, UNKNOWN), IntervalStack::new(1, vec![TWO, THREE]));
}

#[test]
fn test_abstract_stack_0c() {
    let st = IntervalStack::new(0, vec![ONE, UNKNOWN, THREE]);
    assert_eq!(st.values(), vec![ONE, UNKNOWN, THREE]);
    assert_eq!(st.set(2, UNKNOWN), IntervalStack::new(2, vec![THREE]));
}

#[test]
fn test_abstract_stack_0d() {
    let st = IntervalStack::new(1, vec![]);
    assert_eq!(st.set(0, ONE), IntervalStack::new(0, vec![ONE]));
}

#[test]
fn test_abstract_stack_0e() {
    let st = IntervalStack::new(1, vec![TWO]);
    assert_eq!(st.set(1, ONE), IntervalStack::new(0, vec![ONE, TWO]));
}

#[test]
fn test_abstract_stack_0f() {
    let st = IntervalStack::new(2, vec![TWO]);
    assert_eq!(
        st.set(2, ONE),
        IntervalStack::new(0, vec![ONE, UNKNOWN, TWO])
    );
}

#[test]
fn test_abstract_stack_0g() {
    let st = IntervalStack::new(2..=3, vec![TWO]);
    assert_eq!(
        st.set(2, ONE),
        IntervalStack::new(0..=1, vec![ONE, UNKNOWN, TWO])
    );
}

#[test]
fn test_abstract_stack_10() {
    let st1 = IntervalStack::new(1, vec![ONE]);
    let st2 = IntervalStack::new(0, vec![ONE]);
    assert_eq!(st1.join(&st2), IntervalStack::new(0..=1, vec![ONE]));
}
#[test]
fn test_abstract_stack_11() {
    let st1 = IntervalStack::new(0, vec![ONE]);
    let st2 = IntervalStack::new(1, vec![ONE]);
    assert_eq!(st1.join(&st2), IntervalStack::new(0..=1, vec![ONE]));
}
#[test]
fn test_abstract_stack_12() {
    let st1 = IntervalStack::new(0, vec![TWO, ONE]);
    let st2 = IntervalStack::new(1, vec![ONE]);
    assert_eq!(st1.join(&st2), IntervalStack::new(1, vec![ONE]));
}
#[test]
fn test_abstract_stack_13() {
    let st1 = IntervalStack::new(1, vec![ONE]);
    let st2 = IntervalStack::new(0, vec![TWO, ONE]);
    assert_eq!(st1.join(&st2), IntervalStack::new(1, vec![ONE]));
}
#[test]
fn test_abstract_stack_14() {
    let st1 = IntervalStack::new(0, vec![ONE, ONE]);
    let st2 = IntervalStack::new(0, vec![TWO, ONE]);
    assert_eq!(st1.join(&st2), IntervalStack::new(0, vec![ONETWO, ONE]));
}
#[test]
fn test_abstract_stack_15() {
    let st1 = IntervalStack::new(1, vec![ONE, ONE]);
    let st2 = IntervalStack::new(0, vec![TWO, ONE]);
    assert_eq!(st1.join(&st2), IntervalStack::new(0..=1, vec![ONETWO, ONE]));
}
#[test]
fn test_abstract_stack_16() {
    let st1 = IntervalStack::new(0..=1, vec![ONE, ONE]);
    let st2 = IntervalStack::new(0, vec![TWO, ONE]);
    assert_eq!(st1.join(&st2), IntervalStack::new(0..=1, vec![ONETWO, ONE]));
}
#[test]
fn test_abstract_stack_17() {
    let st1 = IntervalStack::new(0, vec![ONE, ONE]);
    let st2 = IntervalStack::new(0..=1, vec![TWO, ONE]);
    assert_eq!(st1.join(&st2), IntervalStack::new(0..=1, vec![ONETWO, ONE]));
}

// Tests for set()
// Force upper expansion
// Force rebalance
