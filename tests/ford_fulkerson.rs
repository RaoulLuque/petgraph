use petgraph::algo::ford_fulkerson;
use petgraph::prelude::Graph;
#[cfg(feature = "stable_graph")]
use petgraph::prelude::{StableDiGraph, StableGraph};
use petgraph::Directed;

#[test]
fn test_ford_fulkerson() {
    // Example from https://downey.io/blog/max-flow-ford-fulkerson-algorithm-explanation/
    let mut graph = Graph::<usize, u16>::new();
    let source = graph.add_node(0);
    let _ = graph.add_node(1);
    let _ = graph.add_node(2);
    let destination = graph.add_node(3);
    graph.extend_with_edges([(0, 1, 3), (0, 2, 2), (1, 2, 5), (1, 3, 2), (2, 3, 3)]);
    let (max_flow, _) = ford_fulkerson(&graph, source, destination);
    assert_eq!(5, max_flow);

    // Example from https://brilliant.org/wiki/ford-fulkerson-algorithm/
    let mut graph = Graph::<usize, f32>::new();
    let source = graph.add_node(0);
    let _ = graph.add_node(1);
    let _ = graph.add_node(2);
    let _ = graph.add_node(3);
    let _ = graph.add_node(4);
    let destination = graph.add_node(5);
    graph.extend_with_edges([
        (0, 1, 4.),
        (0, 2, 3.),
        (1, 3, 4.),
        (2, 4, 6.),
        (3, 2, 3.),
        (3, 5, 2.),
        (4, 5, 6.),
    ]);
    let (max_flow, _) = ford_fulkerson(&graph, source, destination);
    assert_eq!(7.0, max_flow);

    // Example from https://cp-algorithms.com/graph/edmonds_karp.html
    let mut graph = Graph::<usize, f32>::new();
    let source = graph.add_node(0);
    let _ = graph.add_node(1);
    let _ = graph.add_node(2);
    let _ = graph.add_node(3);
    let _ = graph.add_node(4);
    let destination = graph.add_node(5);
    graph.extend_with_edges([
        (0, 1, 7.),
        (0, 2, 4.),
        (1, 3, 5.),
        (1, 4, 3.),
        (2, 1, 3.),
        (2, 4, 2.),
        (3, 5, 8.),
        (4, 3, 3.),
        (4, 5, 5.),
    ]);
    let (max_flow, _) = ford_fulkerson(&graph, source, destination);
    assert_eq!(10.0, max_flow);

    // Example from https://www.programiz.com/dsa/ford-fulkerson-algorithm (corrected: result not 6 but 5)
    let mut graph = Graph::<u8, f32>::new();
    let source = graph.add_node(0);
    let _ = graph.add_node(1);
    let _ = graph.add_node(2);
    let _ = graph.add_node(3);
    let _ = graph.add_node(4);
    let destination = graph.add_node(5);
    graph.extend_with_edges([
        (0, 1, 8.),
        (0, 2, 3.),
        (1, 3, 9.),
        (2, 3, 7.),
        (2, 4, 4.),
        (3, 5, 2.),
        (4, 5, 5.),
    ]);
    let (max_flow, _) = ford_fulkerson(&graph, source, destination);
    assert_eq!(5.0, max_flow);

    let mut graph = Graph::<u8, u8>::new();
    let source = graph.add_node(0);
    let _ = graph.add_node(1);
    let _ = graph.add_node(2);
    let _ = graph.add_node(3);
    let _ = graph.add_node(4);
    let destination = graph.add_node(5);
    graph.extend_with_edges([
        (0, 1, 16),
        (0, 2, 13),
        (1, 2, 10),
        (1, 3, 12),
        (2, 1, 4),
        (2, 4, 14),
        (3, 2, 9),
        (3, 5, 20),
        (4, 3, 7),
        (4, 5, 4),
    ]);
    let (max_flow, _) = ford_fulkerson(&graph, source, destination);
    assert_eq!(23, max_flow);

    // Example taken from https://medium.com/@jithmisha/solving-the-maximum-flow-problem-with-ford-fulkerson-method-3fccc2883dc7
    let mut graph = Graph::<u8, u8>::new();
    let source = graph.add_node(0);
    let _ = graph.add_node(1);
    let _ = graph.add_node(2);
    let _ = graph.add_node(3);
    let _ = graph.add_node(4);
    let destination = graph.add_node(5);
    graph.extend_with_edges([
        (0, 1, 10),
        (0, 2, 10),
        (1, 2, 2),
        (1, 3, 4),
        (1, 4, 8),
        (2, 4, 9),
        (3, 5, 10),
        (4, 3, 6),
        (4, 5, 10),
    ]);
    let (max_flow, _) = ford_fulkerson(&graph, source, destination);
    assert_eq!(19, max_flow);
}

#[cfg(feature = "stable_graph")]
#[test]
fn test_ford_fulkerson_stable_graphs() {
    // See issue https://github.com/petgraph/petgraph/issues/792
    let mut g: StableGraph<(), u32, Directed> = StableDiGraph::new();

    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());

    let ac = g.add_edge(a, c, 1);
    let _ = g.add_edge(a, b, 1);
    let bc = g.add_edge(b, c, 1);
    let _ = g.add_edge(b, d, 1);

    // Current state of graph:
    // a --1-- b --1-- c --1-- d
    // |               |
    // - -- -- 1 -- -- -

    g.remove_edge(bc);

    // Current state of graph:
    // a --1-- b       c --1-- d
    // |               |
    // - -- -- 1 -- -- -

    assert_eq!(1, ford_fulkerson(&g, a, d).0);

    let _ = g.add_edge(b, c, 1);
    g.remove_edge(ac);

    // Current state of graph:
    // a --1-- b --1-- c --1-- d

    assert_eq!(1, ford_fulkerson(&g, a, d).0);

    let _ = g.add_edge(a, c, 1);
    let _ = g.add_edge(c, d, 1);

    // Current state of graph:
    //                 - --1-- -
    //                 |       |
    // a --1-- b --1-- c --1-- d
    // |               |
    // - -- -- 1 -- -- -

    assert_eq!(2, ford_fulkerson(&g, a, d).0);
}
