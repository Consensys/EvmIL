use evmil::util::{SortedVec};

#[test]
pub fn sorted_vec_01() {
    let v : SortedVec<usize> = SortedVec::new();
    let w = vec![];
    assert_eq!(&v,&w);
}

#[test]
pub fn sorted_vec_02() {
    let v : SortedVec<usize> = vec![0].into();
    assert_eq!(&v,&vec![0]);
}

#[test]
pub fn sorted_vec_03() {
    let v : SortedVec<usize> = vec![0,0].into();
    assert_eq!(&v,&vec![0]);
}

#[test]
pub fn sorted_vec_04() {
    let v : SortedVec<usize> = vec![0,1].into();
    assert_eq!(&v,&vec![0,1]);
}

#[test]
pub fn sorted_vec_05() {
    let v : SortedVec<usize> = vec![1,0].into();
    assert_eq!(&v,&vec![0,1]);
}

#[test]
pub fn sorted_vec_06() {
    let v : SortedVec<usize> = vec![1,0,3].into();
    assert_eq!(&v,&vec![0,1,3]);
}

#[test]
pub fn sorted_vec_07() {
    let mut v : SortedVec<usize> = SortedVec::new();
    assert!(v.insert(0));
    assert!(!v.insert(0));
    assert_eq!(&v,&vec![0]);
}

#[test]
pub fn sorted_vec_08() {
    let mut v : SortedVec<usize> = SortedVec::new();
    assert!(v.insert(0));
    assert!(v.insert(1));
    assert_eq!(&v,&vec![0,1]);
}

#[test]
pub fn sorted_vec_09() {
    let mut v : SortedVec<usize> = SortedVec::new();
    assert!(v.insert(1));
    assert!(v.insert(0));
    assert_eq!(&v,&vec![0,1]);
}
