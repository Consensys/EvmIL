use evmil::util::Interval;

#[test]
fn test_interval_01() {
    for lb in 0..10 {
        let r1 = Interval::new(lb,lb);
        assert_eq!(r1.start,lb);
        assert_eq!(r1.end,lb);
        //
        for ub in lb..10 {
            let r1 = Interval::new(lb,ub);
            assert_eq!(r1.start,lb);
            assert_eq!(r1.end,ub);
        }
    }
}

#[test]
fn test_interval_02() {
    let i1 = Interval::from(0..2);
    let i2 = Interval::from(1..3);
    let i3 = Interval::from(3..5);
    //
    assert_eq!(i1.add(1),i2);
    assert_eq!(i2.sub(1),i1);
    assert_eq!(i2.add(2),i3);
}
