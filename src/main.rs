mod graphs;

use graphs::Graph;

fn main() {
    let mut g = Graph::new();
    println!("{:?}", g);
    g.add_vertex();
    g.add_vertex();
    g.add_vertex();
    g.add_vertex();
    g.add_vertex();
    println!("{:?}", g);

}
