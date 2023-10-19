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
use crate::util::{Digraph,Seq,SortedVec};

type DomSet = SortedVec<usize>;

/// Compute the set of nodes dominated by each node in a given graph.
pub fn dominators<T:Seq>(graph: &Digraph<T>) -> Vec<DomSet> {
    // NOTE: this is not particularly efficient
    let n = graph.len();
    // Construct empty relation
    let mut dom : Vec<DomSet> = Vec::new();
    //
    let ns : DomSet = (0..n).collect::<Vec<_>>().into();
    // Initialise dominators
    for i in 0..n {
        if graph.incoming(i).len() == 0 {
            dom.push(vec![i].into());
        } else {
            dom.push(ns.clone());
        }
    }
    //
    let mut changes = true;
    //
    while changes {
        changes = false;
        for i in 0..n {
            for j in graph.incoming(i) {
                if i != *j {
                    // Edge j -> i
                    unsafe {
                        let p : *mut DomSet = &mut dom[i];
                        let q : *const DomSet = &mut dom[*j];
                        changes |= dom_intersect(i,p,q);
                    }
                }
            }
        }
    }    
    //
    dom
}

/// Intersect two sets.  This is safe provided that both pointers are
/// for _different_ sets.
unsafe fn dom_intersect(n: usize, dom: *mut DomSet, pred: *const DomSet) -> bool {
    let l = (*dom).len();
    // Go through every element and check it
    let mut i = 0;
    let mut j = 0;
    //
    while i < (*dom).len() && j < (*pred).len() {
        let ith = (*dom)[i];
        let jth = (*pred)[j];
        //
        if ith == n {
            i += 1;
        } else if ith < jth {
            (*dom).remove_at(i);
        } else if ith > jth {
            j += 1;
        } else {
            i += 1;
            j += 1;
        }
    }
    //
    if i < (*dom).len() { 
        // Remove anything remaining
        (*dom).truncate(i);
        // Put back ourself (in case it was dropped)
        (*dom).insert(n);
    }
    // Check whether anything removed
    l != (*dom).len()
}
