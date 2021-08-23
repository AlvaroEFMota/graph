#[derive(Debug)]
pub struct Graph {
    vector: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl Graph {
    pub fn new() -> Graph {
        Graph {
            vector: vec![]
        }
    }
    pub fn adj(&self, index: usize) -> &Vec<usize> {
        &self.vector[index]
    }

    pub fn add_edge(&mut self, vertex1: usize, vertex2: usize) {
        match self.vector.get(vertex2) {
            Some(_) => (),
            None => panic!("Incapaz de adicionar a aresta, os extremos não existem"),
        };
        let v1 = match self.vector.get_mut(vertex1) {
            Some(i) => i,
            None => panic!("Incapaz de adicionar a aresta, os extremos não existem"),
        };
        v1.push(vertex2);
    }

    pub fn add_vertex(&mut self) -> usize {
        self.vector.push(Vec::new());
        self.vector.len()-1
    }
}