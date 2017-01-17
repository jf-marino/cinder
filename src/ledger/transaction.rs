use std::sync::Arc;
use super::AVL;

#[derive(Debug)]
pub struct LedgerTransaction<V> {
    root: Option<Arc<AVL<String, V>>>
}

impl<V> LedgerTransaction<V> {

    pub fn new(tree: Option<Arc<AVL<String, V>>>) -> LedgerTransaction<V> {
        LedgerTransaction { root: tree }
    }

    pub fn set(&mut self, key: &str, value: V) {
        self.root = match self.root {
            Some(ref tree) => Some(Arc::new(tree.add(String::from(key), value))),
            None => Some(Arc::new(AVL::new(String::from(key), value)))
        };
    }

    pub fn get(&self, key: &str) -> Option<Arc<V>> {
        if let Some(ref tree) = self.root {
            return tree.get(&String::from(key));
        }
        None
    }

    pub fn delete(&mut self, key: &str) {
        self.root = match self.root {
            Some(ref tree) => Some(Arc::new(tree.delete(&String::from(key)))),
            None => None
        };
    }

    pub fn digest(self) -> Option<Arc<AVL<String, V>>> {
        self.root
    }


}