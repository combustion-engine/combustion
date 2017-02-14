//! Default values for `Model` and `Node` types

use common::traits::DefaultName;

use super::data::Node;

impl DefaultName for Node {
    fn default_name() -> String {
        "Untitled Node".to_string()
    }
}