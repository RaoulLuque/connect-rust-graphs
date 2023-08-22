// Graph with generic type
// Nodes are encoded as HashMap<Node<T>,U> U being a value for each node
// Edges are encoded as HashSet<Tuple<Node,Node>>
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;


/// Graph structure where keys are usually primitive like tuples of lists in order to store 
/// gamestates of a game as a graph with e.g. their respective ratings
pub struct Graph<T: Eq + PartialEq + Hash + Copy> {
    /// Set of vertices in the graph
    vertices: HashSet<T>,

    /// Set of edges in the graph
    edges: HashSet<(T, T)>,

    /// Mapping of vertices to their labels
    vertex_labels: HashMap<T, String>,

    /// Mapping of vertices to vector of their inbound neighbours
    inbound_table: HashMap<T, Vec<T>>,

    /// Mapping of vertices to vector of their inbound neighbours
    outbound_table: HashMap<T, Vec<T>>,
}


impl<T: Eq + PartialEq + Hash + Copy> Graph<T> {
    /// Creates a new graph
    pub fn new() -> Graph<T> {
        Graph {
            vertices: HashSet::new(),
            edges: HashSet::new(),
            vertex_labels: HashMap::new(),
            inbound_table: HashMap::new(),
            outbound_table: HashMap::new(),
        }
    }
    
    /// Adds a vertex
    pub fn add_vertex(&mut self, vertex: T) {
        self.vertices.insert(vertex);
    }

    /// Adds a vertex with label
    pub fn add_vertex_with_label(&mut self, vertex: T, label: &str) {
        self.vertices.insert(vertex);
        self.vertex_labels.insert(vertex, label.to_owned());
    }

    /// To do add error when no such vertices present
    pub fn add_edge(&mut self, outgoing: T, incoming: T) {
        self.edges.insert((outgoing, incoming));


        match self.inbound_table.get_mut(&incoming) {
            Some(inbounds) => {inbounds.push(outgoing)},
            None => {
                let mut v: Vec<T> = Vec::new();
                v.push(outgoing);
                self.inbound_table.insert(incoming, v);}
        }

        match self.outbound_table.get_mut(&outgoing) {
            Some(outbounds) => {outbounds.push(incoming)},
            None => {
                let mut v: Vec<T> = Vec::new();
                v.push(incoming);
                self.inbound_table.insert(outgoing, v);}
        }
    }

    /// Returns the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of edges
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Removes a vertex to do: add error if no such vertex is present
    pub fn remove_vertex(&mut self, vertex: &T){
        self.vertices.remove(vertex);
        // Remove outgoing edges with other vertices
        if let Some(outbound) = self.outbound_table.remove(vertex) {
            for other_vertex in outbound {
                self.remove_edge(vertex, &other_vertex);
            }
        }

        // Remove ingoing edges with other vertices
        if let Some(inbound) = self.inbound_table.remove(vertex) {
            for other_vertex in inbound {
                self.remove_edge(&other_vertex, vertex);
            }
        }
    }

    /// Removes an edge to do: add error if no such vertex is present
    pub fn remove_edge(&mut self, inbound: &T, outbound: &T) -> bool {
        self.edges.remove(&(*inbound,*outbound))
    }

    /// Returns an iterator with the ingoing neighbors of the given vertex
    pub fn in_neighbors(&self, vertex: &T) -> std::slice::Iter<'_, T>{
        match self.inbound_table.get(vertex) {
            Some(neighbors) => neighbors.iter(),
            None => [].iter(),
        }
    }

    /// Returns an iterator with the outgoing neighbors of the given vertex
    pub fn out_neighbours(&self, vertex: &T) -> std::slice::Iter<'_, T> {
        match self.inbound_table.get(vertex) {
            Some(neighbors) => neighbors.iter(),
            None => [].iter(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_empty_graph() {
        let g: Graph<u32> = Graph::new();
        assert_eq!(g.edges, HashSet::new());
        assert_eq!(g.vertex_labels, HashMap::new());
        assert_eq!(g.inbound_table, HashMap::new());
        assert_eq!(g.outbound_table, HashMap::new());
    }

    #[test]
    fn adding_vertex_to_graph() {
        let mut g: Graph<u32> = Graph::new();
        g.add_vertex(32);
        assert!(g.vertices.contains(&32));
    }

    #[test]
    fn adding_multiple_vertices() {
        let mut g: Graph<u32> = Graph::new();
        g.add_vertex(32);
        g.add_vertex(1);
        g.add_vertex(2);
        assert!(g.vertices.contains(&32));
        assert!(g.vertices.contains(&1));
        assert!(g.vertices.contains(&2));
        assert!(!g.vertices.contains(&31));
        assert_eq!(g.vertices.len(),3);
        assert_eq!(g.vertex_count(),3);
    }

    #[test]
    fn adding_edges() {
        let mut g: Graph<u32> = Graph::new();
        assert_eq!(g.edge_count(), 0);
        g.add_vertex(2);
        g.add_vertex(3);
        let i = 3;
        g.add_edge(2, i);
        g.add_edge(i, i);
    } 

    #[test]
    fn removing_vertices_and_simultaneously_removing_edges() {
        let mut g: Graph<u32> = Graph::new();
        assert_eq!(g.edge_count(), 0);
        g.add_vertex(2);
        g.add_vertex(3);
        let i = 3;
        g.add_edge(2, i);
        g.add_edge(i, i);
        g.remove_vertex(&i);
        assert!(!g.edges.contains(&(i,i)));
    }

    // to do add test with vertex labels
}
