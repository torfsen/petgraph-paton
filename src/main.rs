use petgraph::{
    csr::IndexType,
    graph::{NodeIndex, UnGraph},
    visit::EdgeRef,
};
use std::{collections::HashMap, fmt::Debug};

fn find_fundamental_set_of_cycles<N, E, Ix>(g: &mut UnGraph<N, E, Ix>) -> Vec<Vec<NodeIndex<Ix>>>
where
    Ix: IndexType + Debug,
{
    let mut cycles = Vec::new();
    // Unless noted otherwise, the variable names in this function follow the variable names in the paper
    if g.node_count() == 0 {
        return cycles;
    }

    // Stack of nodes that are in the spanning tree but which haven't been examined, yet. This has no variable name in the paper.
    let mut unexamined: Vec<_> = Vec::new();

    // Maps a node to their parent in the tree. `T` and `ANC` in the paper.
    let mut anc: HashMap<NodeIndex<Ix>, NodeIndex<Ix>> = HashMap::new();

    fn get_ancestors<Ix>(
        anc: &HashMap<NodeIndex<Ix>, NodeIndex<Ix>>,
        n: NodeIndex<Ix>,
    ) -> Vec<NodeIndex<Ix>>
    where
        Ix: IndexType + Debug,
    {
        let mut n = n;
        let mut ancestors = Vec::new();
        while let Some(&n_anc) = anc.get(&n) {
            let n_anc = n_anc.clone();
            ancestors.push(n_anc.clone());
            n = n_anc;
        }
        ancestors
    }

    fn find_path_in_tree<Ix>(
        anc: &HashMap<NodeIndex<Ix>, NodeIndex<Ix>>,
        start: NodeIndex<Ix>,
        end: NodeIndex<Ix>,
    ) -> Vec<NodeIndex<Ix>>
    where
        Ix: IndexType + Debug,
    {
        let mut start_ancestors = get_ancestors(anc, start);
        let mut end_ancestors = get_ancestors(anc, end);
        //dbg!(&start_ancestors, &end_ancestors);
        let mut path = vec![start];
        let mut last_common_ancestor = None;
        let mut start_ancestor;
        let mut end_ancestor;
        loop {
            start_ancestor = start_ancestors.pop();
            end_ancestor = end_ancestors.pop();
            assert!(!(start_ancestor.is_none() && end_ancestor.is_none()));
            if start_ancestor != end_ancestor {
                break;
            }
            last_common_ancestor = start_ancestor;
            //println!("Common ancestor: {start_ancestor:?}");
        }
        if let Some(start_ancestor) = start_ancestor {
            //dbg!(&start_ancestors, &start_ancestor);
            path.extend(start_ancestors.into_iter());
            path.push(start_ancestor);
        }
        if let Some(last_common_ancestor) = last_common_ancestor {
            path.push(last_common_ancestor);
        }
        if let Some(end_ancestor) = end_ancestor {
            //dbg!(&end_ancestors, &end_ancestor);
            path.extend(end_ancestors.into_iter().rev());
            path.push(end_ancestor);
        }
        path.push(end);
        path
    }

    // The set of nodes in the graph. `X` in the paper.
    let mut x: Vec<_> = g.node_indices().collect();

    // Use an arbitrary node as the root of the tree. We know that `x` has at least 1 element.
    let root = x.last().unwrap().clone();
    unexamined.push(root.clone());

    // We use the "last-element-method" from the paper
    while let Some(z) = unexamined.pop() {
        //println!("{:?}", z);
        for w in g.neighbors(z).collect::<Vec<_>>().into_iter() {
            //println!("    {:?}", w);
            if anc.contains_key(&w) {
                //println!("        Already in t");
                // Since both `z` and `w` are in the tree, there is a unique (undirected) path between them in the tree.
                let cycle = find_path_in_tree(&anc, z, w);
                //println!("CYCLE: {:?}", cycle);
                cycles.push(cycle);
            } else {
                //println!("        Not yet in t");
                anc.insert(w, z);
                unexamined.push(w);
            }

            let edges: Vec<_> = g.edges_connecting(z, w).map(|edge| edge.id()).collect();
            for edge in edges.into_iter() {
                g.remove_edge(edge);
                //println!("        Removing edge between {z:?} and {w:?}");
            }
        }
        let z_index = x.iter().position(|&n| n == z).unwrap();
        x.remove(z_index);
    }
    cycles
}

fn main() {
    let mut g = grid_graph(1000, 1000, 1, 1);
    println!("{} nodes", g.node_count());
    let cycles = find_fundamental_set_of_cycles(&mut g);
    println!("{} cycles", cycles.len());
}


fn grid_graph(cells_x: u32, cells_y: u32, _cell_width: u32, _cell_height: u32) -> UnGraph::<i32, ()> {
    let mut edges = Vec::new();
    for cell_i in 0..(cells_x + 1) {
        //dbg!(cell_i);
        for cell_j in 0..(cells_y + 1) {
            //dbg!(cell_j);
            let lower_left = cell_i * (cells_y + 1) + cell_j;
            let upper_left = lower_left + 1;
            if cell_j < cells_y {
                //dbg!((lower_left, upper_left));
                edges.push((lower_left, upper_left));
            }
            if cell_i < cells_x {
                let lower_right = (cell_i + 1) * (cells_y + 1) + cell_j;
                //dbg!((lower_left, lower_right));
                edges.push((lower_left, lower_right));
            }
        }
    }
    //println!("{:?}", edges);
    UnGraph::<i32, ()>::from_edges(edges)
}