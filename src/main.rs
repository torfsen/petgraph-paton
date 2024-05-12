use petgraph::{
    algo::min_spanning_tree,
    data::FromElements,
    dot::{Config, Dot},
    graph::UnGraph,
};

fn main() {
    let g = UnGraph::<i32, ()>::from_edges([(1, 2), (2, 3), (3, 4), (1, 4)]);

    let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&g));

    println!("{:?}", Dot::with_config(&mst, &[Config::EdgeNoLabel]));
}
