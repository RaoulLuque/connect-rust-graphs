// Graph with generic type
// Nodes are encoded as HashMap<Node<T>,U> U being a value for each node
// Edges are encoded as HashSet<Tuple<Node,Node>>
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

/// Graph structure where keys are usually primitive like tuples of lists in order to store 
/// gamestates of a game as a graph with e.g. their respective ratings
pub struct Graph<T: Eq + PartialEq + Hash> {
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

impl<T: Eq + PartialEq + Hash> Graph<T> {
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
    pub fn add_edge(&mut self, edge: (T,T)) {
        self.edges.insert(edge);
    }

    /// Returns the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of edges
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Removes a vertex
    pub fn remove_vertex(&mut self, vertex: &T) -> bool {
        self.vertices.remove(vertex)
    }

    pub fn remove_edge(&mut self, edge: &(T,T)) -> bool {
        self.edges.remove(&edge)
    }

    pub fn in_neighbours() ->  {

    }

    pub fn out_neighbours() -> {

    }
}