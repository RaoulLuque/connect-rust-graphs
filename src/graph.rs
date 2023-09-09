// Graph with generic type
// Nodes are encoded as HashMap<Node<T>,U> U being a value for each node
// Edges are encoded as HashSet<Tuple<Node,Node>>
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug)]
/// Graph operation error
pub enum GraphError {
    /// There is no vertex with the given id in the graph
    NoSuchVertex,

    /// There is no such edge in the graph
    NoSuchEdge,

}


/// Graph structure where keys are usually primitive like tuples of lists in order to store 
/// gamestates of a game as a graph with e.g. their respective ratings.
/// Vertices and edges are implemented as sets of T and (T,T) tuples respectively and labels are
/// implemented as a hashmap,  just like the adjacency tables of the vertices.
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
    
    /// Adds a vertex to do add error when vertex exists already
    pub fn add_vertex(&mut self, vertex: T) {
        self.vertices.insert(vertex);
    }

    /// Adds a vertex with label to do add error when vertex exists already
    pub fn add_vertex_with_label(&mut self, vertex: T, label: &str) {
        self.vertices.insert(vertex);
        self.vertex_labels.insert(vertex, label.to_owned());
    }

    /// Adds an edge from outgoing to incoming to do add error when edge already exists
    pub fn add_edge(&mut self, outgoing: T, incoming: T) -> Result<(), GraphError> {
        if !self.vertices.contains(&outgoing) || !self.vertices.contains(&incoming) {
            return Err(GraphError::NoSuchVertex);
        }

        self.edges.insert((outgoing, incoming));

        // Add outgoing edge to adjacency table of incoming vertex
        match self.inbound_table.get_mut(&incoming) {
            Some(inbounds) => {inbounds.push(outgoing)},
            None => {
                let mut v: Vec<T> = Vec::new();
                v.push(outgoing);
                self.inbound_table.insert(incoming, v);
            }
        }

        // Add incoming edge to adjacency table of outgoing vertex
        match self.outbound_table.get_mut(&outgoing) {
            Some(outbounds) => {outbounds.push(incoming);},
            None => {
                let mut v: Vec<T> = Vec::new();
                v.push(incoming);
                self.outbound_table.insert(outgoing, v);
            }
        }
        Ok(())
    }

    /// Returns the number of vertices
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of edges
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Removes a vertex to do: remove label from labels
    pub fn remove_vertex(&mut self, vertex: &T) -> Result<(), GraphError> {
        if !self.vertices.contains(vertex) {
            return Err(GraphError::NoSuchVertex);
        }

        // Remove vertex from vertices and remove label if present
        self.vertices.remove(vertex);
        self.vertex_labels.remove(vertex);

        // Remove outgoing edges with other vertices
        if let Some(outbound) = self.outbound_table.remove(vertex) {
            for other_vertex in outbound {
                let _ = self.remove_edge(vertex, &other_vertex);
            }
        }

        // Remove ingoing edges with other vertices
        if let Some(inbound) = self.inbound_table.remove(vertex) {
            for other_vertex in inbound {
                let _ = self.remove_edge(&other_vertex, vertex);
            }
        }
        Ok(())
    }

    /// Removes an edge
    pub fn remove_edge(&mut self, inbound: &T, outbound: &T) -> Result<(), GraphError> {
        if !self.edges.remove(&(*inbound,*outbound)) {
            Ok(())
        } else {
            Err(GraphError::NoSuchEdge)
        }
    }

    /// Returns the label of a vertex as readable reference
    pub fn get_label(&mut self, vertex: &T) -> Option<&String> {
        self.vertex_labels.get(vertex)
    }

    /// Returns the label of a vertex as readable reference
    pub fn get_label_mut(&mut self, vertex: &T) -> Option<&mut String> {
        self.vertex_labels.get_mut(vertex)
    }

    /// Returns an iterator with the ingoing neighbours of the given vertex
    pub fn in_neighbours(&self, vertex: &T) -> std::slice::Iter<'_, T>{
        match self.inbound_table.get(vertex) {
            Some(neighbours) => neighbours.iter(),
            None => [].iter(),
        }
    }

    /// Returns an iterator with the outgoing neighbours of the given vertex
    pub fn out_neighbours(&self, vertex: &T) -> std::slice::Iter<'_, T> {
        match self.outbound_table.get(vertex) {
            Some(neighbours) => neighbours.iter(),
            None => [].iter(),
        }
    }

    /// Returns whether a vertex is in the graph by key
    pub fn is_vertex_in_graph(&self, vertex: &T) -> bool {
        self.vertices.contains(vertex)
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
        g.add_edge(2, i).unwrap();
        g.add_edge(i, i).unwrap();
    } 

    #[test]
    fn removing_vertices_and_simultaneously_removing_edges() {
        let mut g: Graph<u32> = Graph::new();
        assert_eq!(g.edge_count(), 0);
        g.add_vertex(2);
        g.add_vertex(3);
        let i = 3;
        g.add_edge(i, i).unwrap();
        g.add_edge(2, i).unwrap();
        g.remove_vertex(&i).unwrap();
        assert!(!g.edges.contains(&(i,i)));
    }

    #[test]
    fn add_vertices_with_label() {
        let mut g: Graph<u32> = Graph::new();
        assert_eq!(g.vertex_count(), 0);
        g.add_vertex_with_label(1, "A");
        g.add_vertex_with_label(2, "B");
        g.add_vertex(3);
        assert_eq!(g.vertex_count(), 3);

        g.add_vertex_with_label(4,"");
        assert_eq!(g.get_label(&1).unwrap(), "A");

        if let Err(e) = g.remove_vertex(&2) {panic!("Error: {:?}", e);}

        assert_eq!(g.get_label(&2), Option::None);
        assert_eq!(g.get_label(&3), Option::None);
        assert_eq!(g.get_label(&4).unwrap(), "");
    } 

    #[test]
    fn neighbours_in() {
        let mut g: Graph<u32> = Graph::new();
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_vertex(3);
        
        g.add_edge(1,2).unwrap();

        assert_eq!(g.inbound_table.len(),1);
        g.add_edge(1,3).unwrap();
        let v: Vec<&u32> = g.out_neighbours(&1).collect();
        assert_eq!(v.len(), 2);

        assert_eq!(v[0], &2);
    }

    #[test]
    fn is_vertex_in_graph_given_vertex_in_graph_return_true() {
        let mut g: Graph<u32> = Graph::new();
        assert_eq!(g.vertex_count(), 0);
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_vertex(3);
        assert_eq!(g.vertex_count(), 3);

        assert!(g.is_vertex_in_graph(&1));
        assert!(g.is_vertex_in_graph(&2));
        assert!(g.is_vertex_in_graph(&3));
    }

    #[test]
    fn is_vertex_in_graph_given_vertex_not_in_graph_return_false() {
        let mut g: Graph<u32> = Graph::new();
        assert_eq!(g.vertex_count(), 0);
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_vertex(3);
        assert_eq!(g.vertex_count(), 3);

        assert!(!g.is_vertex_in_graph(&4));
        assert!(!g.is_vertex_in_graph(&6));
    } 
}
