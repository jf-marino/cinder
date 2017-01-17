#[allow(dead_code, non_snake_case)]
mod avl;
#[allow(dead_code, non_snake_case)]
mod transaction;
#[allow(dead_code, non_snake_case)]
mod state;

pub use self::avl::AVL;
pub use self::transaction::LedgerTransaction;
pub use self::state::State;

use std::ptr;
use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};


#[allow(dead_code, non_snake_case)]
#[derive(Debug)]
pub struct Ledger<V> {
    head: AtomicPtr<State<V>>
}

#[allow(dead_code, non_snake_case)]
impl<V> Ledger<V> {

    fn stateRef(value: State<V>) -> *mut State<V> {
        Box::into_raw(Box::new(value))
    }

    fn runTransaction<F>(&self, f: Arc<F>) -> bool where F: Fn(LedgerTransaction<V>) -> LedgerTransaction<V> {
        let original = self.head.load(Ordering::Relaxed);
        let state = unsafe { ptr::read(original) };

        let root = match state.map() {
            Some(ref tree) => Some(tree.clone()),
            None => None
        };

        let mut t = LedgerTransaction::new(root);
        t = f(t);

        let newtree = t.digest();
        if newtree.is_some() {
            let newstate = Ledger::stateRef(State::<V>::new(newtree, original));
            return self.head.compare_and_swap(original, newstate, Ordering::Relaxed) == original
        }
        return true;
    }

    pub fn new() -> Ledger<V> {
        Ledger { head: AtomicPtr::new(Ledger::stateRef(State::<V>::empty())) }
    }

    pub fn load(&self) -> State<V> {
        unsafe { ptr::read(self.head.load(Ordering::Relaxed)) }
    }

    pub fn attemptTransaction<F>(&self, retry: u32, f: F) -> bool where F: Fn(LedgerTransaction<V>) -> LedgerTransaction<V> {
        let cls = Arc::new(f);
        for _ in 0..(retry + 1) {
            let succeeded = self.runTransaction(cls.clone());
            if succeeded { return true }
        }
        false
    }

    pub fn transaction<F>(&self, f: F) -> bool where F: Fn(LedgerTransaction<V>) -> LedgerTransaction<V> {
        return self.attemptTransaction(3, f);
    }
}

#[cfg(test)]
mod test {
    use super::Ledger;

    #[test]
    fn can_add_values() {
        let led = Ledger::<i32>::new();

        led.transaction(|mut t| {
            t.set("foo", 20);
            t.set("bar", 10);
            t.set("baz", -1000);
            t
        });

        if let Some(ref tree) = led.load().map() {
            println!("{:?}", tree.lineup());
        }

        led.transaction(|mut t| {
            t.delete("baz");
            t
        });


        if let Some(ref tree) = led.load().map() {
            println!("{:?}", tree.lineup());
        }
    }
}
