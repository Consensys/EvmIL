use evmil::util::{Bottom, Interval, Join, Top};

const TOP: Interval<usize> = Interval::TOP;
const BOTTOM: Interval<usize> = Interval::BOTTOM;

#[test]
fn test_interval_01() {
    for lb in 0..10 {
        let r1 = Interval::new(lb, lb);
        assert_eq!(r1.start, lb);
        assert_eq!(r1.end, lb);
        //
        for ub in lb..10 {
            let r1 = Interval::new(lb, ub);
            assert_eq!(r1.start, lb);
            assert_eq!(r1.end, ub);
        }
    }
}

#[test]
fn test_interval_02() {
    let i1 = Interval::from(0..=2);
    let i2 = Interval::from(1..=3);
    let i3 = Interval::from(3..=5);
    //
    assert_eq!(i1 + 1, i2);
    assert_eq!(i2 - 1usize, i1);
    assert_eq!(i2 + 2, i3);
}

#[test]
fn test_interval_03() {
    let i1 = Interval::<usize>::from(0..=2);
    //
    assert_eq!(i1 + TOP, TOP);
    assert_eq!(TOP + i1, TOP);
}

#[test]
fn test_interval_04() {
    let i1 = Interval::<usize>::from(1..=2);
    //
    assert_eq!(i1.join(&BOTTOM), i1);
    assert_eq!(BOTTOM.join(&i1), i1);
}
