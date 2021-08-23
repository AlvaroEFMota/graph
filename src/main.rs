#[derive(Debug)]
struct Graph {
    vector: Vec<Vec<usize>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            vector: vec![]
        }
    }
    fn adj(&self, index: usize) -> &Vec<usize> {
        &self.vector[index]
    }

    fn add_edge(&mut self, vertex1: usize, vertex2: usize) {
        match self.vector.get(vertex2) {
            Some(i) => i,
            None => panic!("Incapaz de adicionar a aresta, os extremos não existem"),
        };
        let v1 = match self.vector.get_mut(vertex1) {
            Some(i) => i,
            None => panic!("Incapaz de adicionar a aresta, os extremos não existem"),
        };
        v1.push(vertex2);
    }

    fn add_vertex(&mut self) -> usize {
        self.vector.push(Vec::new());
        self.vector.len()-1
    }
}

fn main() {
    let mut g = Graph{
        vector: vec![vec![1, 2], vec![0, 2], vec![0, 1]],
    };
    println!("{:?}", g);
    println!("{:?}", g.adj(0));
    println!("added vertex: {}", g.add_vertex());
    g.add_edge(3, 0);
    println!("{:?}", g);

    let mut gg = Graph::new();
    println!("{:?}", gg);
    gg.add_vertex();
    gg.add_vertex();
    gg.add_vertex();
    gg.add_vertex();
    gg.add_vertex();
    println!("{:?}", gg);

}
