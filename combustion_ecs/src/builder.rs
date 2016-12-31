//! Acyclic Directed graph dependency builder for

use std::collections::hash_map::Entry;

use fnv::FnvHashMap;
use specs;

use petgraph::prelude::*;
use petgraph::algo::*;
use petgraph::visit::*;

use error::*;

pub type SystemConstructor = Box<FnMut(&mut super::Planner, specs::Priority) -> SystemResult<()>>;

type SystemGraph = Graph<Option<SystemConstructor>, (), Directed, usize>;

pub struct SystemBuilder {
    node_table: FnvHashMap<String, NodeIndex<usize>>,
    root: NodeIndex<usize>,
    graph: SystemGraph,
    cycle_state: DfsSpace<NodeIndex<usize>, < SystemGraph as Visitable >::Map>,
}

impl SystemBuilder {
    pub fn new() -> SystemBuilder {
        let mut graph = Graph::default();
        let root = graph.add_node(None);

        SystemBuilder { node_table: FnvHashMap::default(), root: root, graph: graph, cycle_state: DfsSpace::default() }
    }

    fn add_system_impl(&mut self, name: String, constructor: SystemConstructor) -> SystemResult<NodeIndex<usize>> {
        Ok(match self.node_table.entry(name.into()) {
            Entry::Occupied(occupied_entry) => {
                let node = occupied_entry.get().clone();

                // We already have the node index for this system, so it definitely exists.
                // Overwrite the previous constructor with the new one
                if let Some(mut weight) = self.graph.node_weight_mut(node) {
                    *weight = Some(constructor);
                } else {
                    // If for some really weird reason the system existed in the node_table but not in the graph, complain about it.
                    return Err(SystemError::DuplicateSystem(occupied_entry.key().clone()));
                }

                node
            },
            Entry::Vacant(vacant_entry) => {
                // If the system didn't exist, add it to the graph and place the node index in the vacant entry in the node_table
                let node = self.graph.add_node(Some(constructor));

                vacant_entry.insert(node);

                node
            }
        })
    }

    pub fn add_system<S: Into<String>>(&mut self, name: S, constructor: SystemConstructor) -> SystemResult<NodeIndex<usize>> {
        let node = self.add_system_impl(name.into(), constructor)?;

        // Connect a system with zero dependencies to the root node
        self.graph.add_edge(self.root, node, ());

        Ok(node)
    }

    pub fn add_system_with_deps<S: Into<String>, D: IntoIterator<Item = String>>(&mut self, name: S, constructor: SystemConstructor, deps: D) -> SystemResult<NodeIndex<usize>> {
        let node = self.add_system_impl(name.into(), constructor)?;

        for dep in deps.into_iter() {
            let dep_node = match self.node_table.entry(dep) {
                Entry::Vacant(vacant_entry) => {
                    let dep_name = vacant_entry.key().clone();

                    let dep_node = self.graph.add_node(Some(box move |_, _| {
                        Err(SystemError::MissingDependentSystem(dep_name.clone()))
                    }));

                    self.graph.add_edge(self.root, dep_node, ());

                    vacant_entry.insert(dep_node);

                    dep_node
                },
                Entry::Occupied(occupied_entry) => {
                    let dep_node = occupied_entry.get().clone();

                    if has_path_connecting(&self.graph, dep_node, node, Some(&mut self.cycle_state)) {
                        return Err(SystemError::WouldCycle);
                    }

                    dep_node
                }
            };

            self.graph.add_edge(dep_node, node, ());
        }

        Ok(node)
    }

    pub fn build(mut self, mut planner: &mut super::Planner) -> SystemResult<()> {
        let mut bfs = Dfs::new(&self.graph, self.root);

        // Since specs has a higher-number = higher-priority sorting policy, we need to start from the max value and go from highest priority systems to lowest.
        let mut priority = specs::Priority::max_value();

        while let Some(node) = bfs.next(&self.graph) {
            if let &mut Some(ref mut cb) = self.graph.node_weight_mut(node).unwrap() {
                cb(planner, priority)?;

                priority -= 1;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use ::Planner;

    #[test]
    fn basic() {
        let mut builder = SystemBuilder::new();

        macro_rules! dummy {
            ($name:expr) => {box |_, p| {
                println!("Name: {} {}", $name, p);

                Ok(())
            }}
        }

        macro_rules! deps {
            [$($dep:expr),*] => {[$($dep),*].iter().map(|s| s.to_string())}
        }

        builder.add_system("test", dummy!("test")).unwrap();
        builder.add_system("testing", dummy!("testing")).unwrap();

        builder.add_system_with_deps("test1", dummy!("test1"), deps!["testing"]).unwrap();
        builder.add_system_with_deps("test4", dummy!("test4"), deps!["test"]).unwrap();
        builder.add_system_with_deps("test3", dummy!("test3"), deps!["test2"]).unwrap();
        builder.add_system_with_deps("test5", dummy!("test5"), deps!["test2"]).unwrap();
        builder.add_system_with_deps("test2", dummy!("test2"), deps!["test4"]).unwrap();

        let mut planner = {
            Planner::new(specs::World::new(), 4)
        };

        builder.build(&mut planner).unwrap();
    }
}