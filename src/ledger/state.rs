use std::ptr;
use std::sync::Arc;
use super::AVL;

#[derive(Debug)]
pub struct State<V> {
    previous: *const State<V>,
    tree: Option<Arc<AVL<String, V>>>
}

impl<V> State<V> {

    pub fn empty() -> State<V> {
        State { previous: ptr::null(), tree: None }
    }

    pub fn new(tree: Option<Arc<AVL<String, V>>>, previous: *const State<V>) -> State<V> {
        State { previous: previous, tree: tree }
    }

    pub fn map(&self) -> Option<Arc<AVL<String, V>>> {
        self.tree.clone()
    }

}