use crate::{data_base::{Reachdb, UserDefinedRelationType}, errors::ReachdbError};
use log::{debug, info};
use rand::Rng;


impl<E: UserDefinedRelationType + std::fmt::Debug> Reachdb<E> {
    /// Returns the relation ids
    pub fn random_walk(&self, start_node_id: u64, steps: usize) -> Result<Vec<u64>, ReachdbError> {
        let mut current_node = start_node_id;
        let mut path = vec![];
        info!("Starting random walk from node: {}", current_node);
        for _ in 0..steps {
            debug!("Current node: {}", current_node);
            let relations = self.get_outgoing_node_relations(current_node)?;
            if relations.is_empty() {
                return Ok(path);
            }
            let random_index = rand::thread_rng().gen_range(0..relations.len());
            let relation = relations[random_index];
            let next_node = self.get_connected_node(current_node, relation)?;
            path.push(relation);
            current_node = next_node;
        }
        Ok(path)
    }
}