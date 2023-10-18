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
use crate::util::{Seq,SortedVec};

type EdgeSet = SortedVec<usize>;

/// Represents a _bidirectional_ directed graph over a sequence of
/// _nodes_.  Edges can be added/removed, and iterated over in the
/// _forwards_ or _backwards_ direction.  The underlying datastructure
/// is that of an adjacency list.
pub struct Digraph<T>
where T:Seq {
    /// Node sequence underlying this representation.
    nodes: T,
    /// For each block, the corresponding set of incoming edges
    /// (i.e. edges which transfer control into this block).
    incoming: Vec<EdgeSet>,
    /// For each block, the corresponding set of outgoing edges
    /// (i.e. edges which transfer control out of this block).
    outgoing: Vec<EdgeSet>    
}

impl<T> Digraph<T>
where T:Seq {
    pub fn new(n: usize, nodes: T) -> Self {
        let incoming = vec![SortedVec::new();n];
        let outgoing = vec![SortedVec::new();n];
        Self{nodes,incoming,outgoing}
    }
    
    /// Returns the number of nodes stored within this graph.    
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns `true` if the node set for this graph is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    
    /// Returns the ith block within this graph.    
    pub fn get(&self, index: usize) -> T::Output {
        self.nodes.get(index).unwrap()
    }
    
    /// Get the underlying nodes of this graph (in whatever form they
    /// are provided).
    pub fn nodes(&self) -> &T {
        &self.nodes
    }
    
    /// Returns the set of blocks which can transfer control _into_ a
    /// given block (`blk`).    
    pub fn incoming(&self, blk: usize) -> &[usize] {
        &self.incoming[blk]        
    }

    /// Returns the set of blocks to which a given block (`blk`) can
    /// transfer control.    
    pub fn outgoing(&self, blk: usize) -> &[usize] {
        &self.outgoing[blk]
    }

    /// Returns an iterator over the _outgoing edges_ in this graph
    pub fn out_iter(&self) -> DigraphIterator {
        DigraphIterator::new(true,&self.outgoing)
    }

    /// Returns an iterator over the _incoming edges_ in this graph
    pub fn in_iter(&self) -> DigraphIterator {
        DigraphIterator::new(false,&self.incoming)
    }    
    
    /// Connect one basic block to another which forms a directed edge
    /// in the graph.  Returns `true` if a connection was added.
    pub fn connect(&mut self, from: usize, to: usize) -> bool {
        self.incoming[to].insert(from);
        self.outgoing[from].insert(to)
    }

    /// Remove a connection between two basic blocks from the graph.
    /// Returns `true` if a connection was removed.
    pub fn disconnect(&mut self, from: usize, to: usize) -> bool {
        self.incoming[to].remove(&from);
        self.outgoing[from].remove(&to)
    }
}

/// An iterator over the edges of a graph which iterates over the
/// incoming or outgoing edges of the graph.
pub struct DigraphIterator<'a> {
    // Direction of edges which is either `true` (i.e. outgoing) or
    // `false` (i.e. incoming).
    dir: bool,
    // Edge sets being iterated over.
    items: &'a [EdgeSet],
    // Current edgeset being considered.
    i: usize,
    // Current position within edgeset being considered.
    j: usize 
}

impl<'a> DigraphIterator<'a> {
    pub fn new(dir: bool, items: &'a [EdgeSet]) -> Self {
        Self{dir,items,i:0,j:0}
    }
}

impl<'a> Iterator for DigraphIterator<'a> {
    // An edge
    type Item = (usize,usize);

    fn next(&mut self) -> Option<Self::Item> {
        //
        while self.i < self.items.len() {
            let head = &self.items[self.i];
            // sanity check position
            if self.j >= head.len() {
                self.j = 0;
                self.i += 1;
            } else {
                // Found an edge
                let k = head[self.j];
                self.j += 1;
                // Construct edge
                let edge = if self.dir { (self.i,k) } else { (k,self.i) };
                // Done
                return Some(edge);
            }
        }
        // Empty
        None
    }
}


