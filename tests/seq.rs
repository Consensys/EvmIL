// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use evmil::util::Seq;

struct Wrapper<S:Seq> {
    items: S
}

impl<S:Seq> Wrapper<S> {
    fn len(&self) -> usize { self.items.len() }
    
    fn get(&self, i: usize) -> S::Output {
        self.items.get(i).unwrap()
    }
}

    
    
fn get<S:Seq+?Sized>(s: &S, index: usize) -> S::Output {
    s.get(index).unwrap()
}

// #[test]
// fn test_vec_seq_01() {
//     let v = vec![1];
//     assert_eq!(get(&v,0),&1);
//     assert_eq!(v.len(),1);    
// }

// #[test]
// fn test_vec_seq_02() {
//     let v = vec![1,2,3];
//     assert_eq!(get(&v,0),&1);
//     assert_eq!(get(&v,1),&2);
//     assert_eq!(get(&v,2),&3);    
//     assert_eq!(v.len(),3);        
// }

#[test]
fn test_slice_seq_01() {
    let v : &[_] = &[1];
    assert_eq!(get(v,0),1);
    assert_eq!(v.len(),1);
}

#[test]
fn test_slice_seq_02() {
    let v = &[1];
    assert_eq!(get(v,0),1);
    assert_eq!(v.len(),1);
}

#[test]
fn test_slice_seq_03() {
    let slice : &[_] = &[1,2,3];
    let w = Wrapper{items: slice};
    assert_eq!(w.get(0),&1);
    assert_eq!(w.get(1),&2);
    assert_eq!(w.get(2),&3);
    assert_eq!(w.len(),3);
}

#[test]
fn test_slice_seq_04() {
    let w = Wrapper{items: &[1,2,3]};
    assert_eq!(w.get(0),&1);
    assert_eq!(w.get(1),&2);
    assert_eq!(w.get(2),&3);
    assert_eq!(w.len(),3);    
}
