// Graph with generic type
// Nodes are encoded as HashMap<Node<T>,U> U being a value for each node
// Edges are encoded as HashSet<Tuple<Node,Node>>
use std::collections::HashMap;
use std::collections::HashSet;



pub struct Graph<T> {
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

impl<T> Graph<T> {
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
    
    pub fn add_vertex() {

    }

    pub fn add_edge() {

    }

    pub fn neighbours() {

    }

    pub fn vertex_count() {

    }

    pub fn edge_count() {

    }

    pub fn remove_vertex() {

    }

    pub fn remove_edge() {

    }

    pub fn in_neighbours() ->  {

    }

    pub fn out_neighbours() -> {
        
    }
}