use evmil::util::{Digraph,dominators};

const nodes :&[usize] = &[0,1,2,3,4];

#[test]
pub fn dominators_01() {
    let edges = &[(0,1),(1,2)];
    let dom : &[&[usize]] = &[&[0],&[0,1],&[0,1,2],&[3],&[4]];
    //
    check(edges, dom);
}

#[test]
pub fn dominators_02() {
    let edges = &[(0,1),(1,2),(0,3),(2,4)];
    let dom : &[&[usize]] = &[&[0],&[0,1],&[0,1,2],&[0,3],&[0,1,2,4]];
    //
    check(edges, dom);
}

#[test]
pub fn dominators_03() {
    let edges = &[(0,1),(1,2),(0,3),(2,4),(3,4)];
    let dom : &[&[usize]] = &[&[0],&[0,1],&[0,1,2],&[0,3],&[0,4]];
    //
    check(edges, dom);
}

#[test]
pub fn dominators_04() {
    let edges = &[(0,1),(1,2),(2,3),(3,1),(1,4)];
    let dom : &[&[usize]] = &[&[0],&[0,1],&[0,1,2],&[0,1,2,3],&[0,1,4]];
    //
    check(edges, dom);
}

fn check(edges: &[(usize,usize)], dom: &[&[usize]]) {
    let graph = from_edges(edges);
    let ds = dominators(&graph);
    //
    assert_eq!(ds.len(),dom.len());
    //
    for i in 0..ds.len() {
        let ith : &[usize] = &ds[i];
        assert_eq!(dom[i],ith);
    }
}

fn from_edges(edges: &[(usize,usize)]) -> Digraph<&'static [usize]> {
    let mut graph = Digraph::new(nodes.len(),nodes);
    for (n,m) in edges {
        graph.connect(*n,*m);
    }
    graph
}
