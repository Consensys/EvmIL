use evmil::util::{Digraph,transitive_closure};

const nodes :&[usize] = &[0,1,2,3,4];

#[test]
pub fn transitive_closure_01() {
    let edges = &[(0,1),(1,2)];
    let tc : &[&[usize]] = &[&[1,2],&[2],&[],&[],&[]];
    //
    check(edges, tc);
}

#[test]
pub fn transitive_closure_02() {
    let edges = &[(0,1),(1,2),(0,3),(2,4)];
    let tc : &[&[usize]] = &[&[1,2,3,4],&[2,4],&[4],&[],&[]];
    //
    check(edges, tc);
}

#[test]
pub fn transitive_closure_03() {
    let edges = &[(0,1),(1,2),(0,3),(2,4),(3,4)];
    let tc : &[&[usize]] = &[&[1,2,3,4],&[2,4],&[4],&[4],&[]];
    //
    check(edges, tc);
}

#[test]
pub fn transitive_closure_04() {
    let edges = &[(0,1),(1,2),(2,3),(3,1),(1,4)];
    let tc : &[&[usize]] = &[&[1,2,3,4],&[1,2,3,4],&[1,2,3,4],&[1,2,3,4],&[]];
    //
    check(edges, tc);
}

fn check(edges: &[(usize,usize)], tc: &[&[usize]]) {
    let graph = from_edges(edges);
    let ds = transitive_closure(&graph);
    //
    assert_eq!(ds.len(),tc.len());
    //
    for i in 0..ds.len() {
        let ith : &[usize] = &ds[i];
        assert_eq!(tc[i],ith);
    }
}

fn from_edges(edges: &[(usize,usize)]) -> Digraph<&'static [usize]> {
    let mut graph = Digraph::new(nodes.len(),nodes);
    for (n,m) in edges {
        graph.connect(*n,*m);
    }
    graph
}
