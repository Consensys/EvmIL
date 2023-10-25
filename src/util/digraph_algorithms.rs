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
use crate::util::{Digraph,EdgeSet,Seq,SortedVec};

// ===================================================================
// Dominators
// ===================================================================

/// Compute the set of nodes dominated by each node in a given graph.
pub fn dominators<T:Seq>(graph: &Digraph<T>) -> Vec<EdgeSet> {
    // NOTE: this is not particularly efficient
    let n = graph.len();
    // Construct empty relation
    let mut dom : Vec<EdgeSet> = Vec::new();
    //
    let ns : EdgeSet = (0..n).collect::<Vec<_>>().into();
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
                        let p : *mut EdgeSet = &mut dom[i];
                        let q : *const EdgeSet = &mut dom[*j];
                        changes |= dom_intersect(i,p,q);
                    }
                }
            }
        }
    }    
    //
    dom
}

// ===================================================================
// Transitive Closure
// ===================================================================

/// Compute the (forward) transitive closure of a graph.  That is, for
/// each node, the set of nodes it can reach in one (or more) hops.
pub fn transitive_closure<T:Seq>(graph: &Digraph<T>) -> Vec<EdgeSet> {
    let mut changed = true;    
    let mut closure = Vec::new();
    // Initialise the closure
    for i in 0..graph.len() {
        closure.push(graph.outgoing(i).clone());
    }
    // Iterate to a fixed point
    while changed {
        changed = false;
        //
        for i in 0..graph.len() {
            for j in graph.outgoing(i) {
                if i != *j {
                    unsafe {
                        // This is safe because we know that i != j, and
                        // hence the two sets are actually disjoint.
                        let l : *mut EdgeSet = &mut closure[i];
                        let r : *const EdgeSet = &mut closure[*j];
                        changed |= (*l).insert_all(&(*r));
                    }
                }
            }
        }
    }
    //
    closure
}

// ===================================================================
// Helpers
// ===================================================================

/// Intersect two sets.  This is safe provided that both pointers are
/// for _different_ sets.
unsafe fn dom_intersect(n: usize, dom: *mut EdgeSet, pred: *const EdgeSet) -> bool {
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
