use std::cmp::{Ord};
use std::sync::Arc;
use std::clone::Clone;

fn max(first: u32, second: u32) -> u32 {
    if first >= second { first } else { second }
}

#[derive(Debug)]
pub enum Balance {
    Left,
    Right,
    None
}

#[derive(Debug, PartialEq, Eq)]
pub struct AVL<K, V> where K: Eq + Ord {
    key: Arc<K>,
    value: Arc<V>,
    lheight: u32,
    rheight: u32,
    left: Option<Arc<AVL<K, V>>>,
    right: Option<Arc<AVL<K, V>>>
}

impl<K, V> Clone for AVL<K, V> where K: Eq + Ord {
    fn clone(&self) -> AVL<K, V> {
        AVL {
            key: self.key.clone(),
            value: self.value.clone(),
            lheight: self.lheight,
            rheight: self.rheight,
            left: self.left.clone(),
            right: self.right.clone()
        }
    }
}

impl<K, V> AVL<K, V> where K: Eq + Ord {

    fn cp(&self) -> AVL<K, V> {
        self.clone()
    }

    fn accumulate(&self, acc: &mut Vec<(Arc<K>, Arc<V>)>) {
        if let Some(ref left) = self.left {
            left.accumulate(acc);
        }
        acc.push((self.key.clone(), self.value.clone()));
        if let Some(ref right) = self.right {
            right.accumulate(acc);
        }
    }

    pub fn lineup(&self) -> Vec<(Arc<K>, Arc<V>)> {
        let mut v = Vec::<(Arc<K>, Arc<V>)>::new();
        self.accumulate(&mut v);
        return v;
    }

    fn isBalanced(&self) -> Balance {
        let b: i32 = self.rheight as i32 - self.lheight as i32;
        if b < -1 {
            return Balance::Left;
        }
        if b > 1 {
            return Balance::Right;
        }
        Balance::None
    }

    fn hasLeft(&self) -> bool {
        self.right.is_some()
    }

    fn hasRight(&self) -> bool {
        self.left.is_some()
    }

    fn shiftRightmost(&self) -> ((Arc<K>, Arc<V>), Option<Arc<AVL<K, V>>>) {
        if let Some(ref right) = self.right {
            let mut cloned = self.clone();
            let (rightmost, tree) = right.shiftRightmost();
            cloned.rheight = match tree {
                Some(ref r) => max(r.lheight, r.rheight),
                None => 0
            };
            cloned.right = tree;
            return (rightmost, Some(Arc::new(cloned)));
        }
        let k = self.key.clone();
        let v = self.value.clone();
        let left = self.left.clone();
        ((k, v), left)
    }

    fn shiftLeftmost(&self) -> ((Arc<K>, Arc<V>), Option<Arc<AVL<K, V>>>) {
        if let Some(ref left) = self.left {
            let mut cloned = self.clone();
            let (leftmost, tree) = left.shiftLeftmost();
            cloned.lheight = match tree {
                Some(ref l) => max(l.lheight, l.rheight),
                None => 0
            };
            cloned.left = tree;
            return (leftmost, Some(Arc::new(cloned)));
        }
        let k = self.key.clone();
        let v = self.value.clone();
        let right = self.right.clone();
        ((k, v), right)
    }

    fn balanceLeft(&self) -> AVL<K, V> {
        let mut cloned = self.clone();
        if let Some(ref left) = self.left {
            let mut leftClone = left.cp();
            if left.left.is_some() {
                let mut tree = leftClone;
                cloned.left = None;
                cloned.lheight = 0;
                cloned.rheight = 0;
                tree.rheight = 1;
                tree.lheight = 1;
                tree.right = Some(Arc::new(cloned));
                return tree;
            }
            if let Some(ref r) = left.right {
                let mut tree = r.cp();
                cloned.right = None;
                cloned.left = None;
                cloned.lheight = 0;
                cloned.rheight = 0;
                tree.right = Some(Arc::new(cloned));

                leftClone.right = None;
                leftClone.rheight = 0;

                tree.left = Some(Arc::new(leftClone));
                tree.lheight = 1;
                tree.rheight = 1;
                return tree;
            }
        }
        cloned
    }

    fn balanceRight(&self) -> AVL<K, V> {
        let mut cloned = self.clone();
        if let Some(ref right) = self.right {
            let mut rightClone = right.cp();
            if right.right.is_some() {
                let mut tree = rightClone;
                cloned.right = None;
                cloned.rheight = 0;
                cloned.lheight = 0;
                tree.lheight = 1;
                tree.rheight = 1;
                tree.left = Some(Arc::new(cloned));
                return tree;
            }
            if let Some(ref l) = right.left {
                let mut tree = l.cp();
                cloned.right = None;
                cloned.left = None;
                cloned.lheight = 0;
                cloned.rheight = 0;
                tree.left = Some(Arc::new(cloned));

                rightClone.right = None;
                rightClone.rheight = 0;

                tree.right = Some(Arc::new(rightClone));
                tree.lheight = 1;
                tree.rheight = 1;
                return tree;
            }
        }
        cloned
    }


    pub fn new(key: K, value: V) -> AVL<K, V> {
        AVL { key: Arc::new(key), value: Arc::new(value), lheight: 0, rheight: 0, left: None, right: None }
    }

    pub fn val(&self) -> &V {
        self.value.as_ref()
    }

    pub fn add(&self, key: K, value: V) -> AVL<K, V> {
        let mut cloned = self.clone();

        if &key == self.key.as_ref() {
            cloned.value = Arc::new(value);
            return cloned;
        }

        if &key < self.key.as_ref() {
            let nleft;
            if let Some(ref l) = self.left {
                nleft = l.add(key, value);
            } else {
                nleft = AVL::new(key, value);
            }
            cloned.lheight = max(nleft.lheight, nleft.rheight) + 1;
            cloned.left = Some(Arc::new(nleft));
        } else {
            let nright;
            if let Some(ref r) = self.right {
                nright = r.add(key, value);
            } else {
                nright = AVL::new(key, value);
            }
            cloned.rheight = max(nright.lheight, nright.rheight) + 1;
            cloned.right = Some(Arc::new(nright));
        }

        match cloned.isBalanced() {
            Balance::Left => cloned.balanceLeft(),
            Balance::Right => cloned.balanceRight(),
            Balance::None => cloned
        }
    }

    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        if key == self.key.as_ref() {
            return Some(self.value.clone());
        }
        if key < self.key.as_ref() {
            if let Some(ref left) = self.left {
                return left.get(key);
            }
        }
        if key > self.key.as_ref() {
            if let Some(ref right) = self.right {
                return right.get(key);
            }
        }
        None
    }

    pub fn delete(&self, key: &K) -> AVL<K, V> {
        let mut cloned = self.clone();
        if key < self.key.as_ref() {
            if let Some(ref left) = self.left {
                let newleft = left.delete(key);
                cloned.lheight = max(newleft.lheight, newleft.rheight) + 1;
                cloned.left = Some(Arc::new(newleft));
            }
            return cloned;
        };

        if key > self.key.as_ref() {
            if let Some(ref right) = self.right {
                let newright = right.delete(key);
                cloned.rheight = max(newright.lheight, newright.rheight) + 1;
                cloned.right = Some(Arc::new(newright));
            }
            return cloned;
        };

        if key == self.key.as_ref() {
            if let Some(ref left) = self.left {
                let ((key, value), tree) = left.shiftRightmost();
                cloned.key = key;
                cloned.value = value;
                cloned.lheight = match tree {
                    Some(ref newleft) => max(newleft.lheight, newleft.rheight) + 1,
                    None => 0
                };
                cloned.left = tree;
            } else if let Some(ref right) = self.right {
                let ((key, value), tree) = right.shiftLeftmost();
                cloned.key = key;
                cloned.value = value;
                cloned.rheight = match tree {
                    Some(ref newright) => max(newright.lheight, newright.rheight) + 1,
                    None => 0
                };
                cloned.right = tree;
            }
        };

        return cloned
    }
}


#[cfg(test)]
mod test {
    use super::AVL;

    #[test]
    fn can_create_avl() {
        let a = AVL::new(String::from("hello"), 10);
        assert_eq!(a.key.as_ref(), "hello");
        assert_eq!(a.value.as_ref(), &10);
    }

    #[test]
    fn can_add_value() {
        let a = AVL::new(String::from("hello"), 10);
        let b = a.add(String::from("foo"), 22);
        let c = b.add(String::from("bar"), 33);

        assert_eq!(c.key.as_ref(), "foo");
        assert_eq!(c.left.unwrap().key.as_ref(), "bar");
        assert_eq!(c.right.unwrap().key.as_ref(), "hello");
    }

    #[test]
    fn can_remove_value() {
        let a = AVL::new(String::from("hello"), 10);
        let b = a.add(String::from("foo"), 22);
        let c = b.add(String::from("bar"), 33);
        let d = c.add(String::from("baz"), 44);
        let e = d.add(String::from("norm"), 55);
        let f = e.delete(&String::from("foo"));

        assert_eq!(f.key.as_ref(), "baz");
        assert_eq!(f.left.unwrap().key.as_ref(), "bar");
        if let Some(ref right) = f.right {
            assert_eq!(right.key.as_ref(), "hello");
            if let Some(ref rightright) = right.right {
                assert_eq!(rightright.key.as_ref(), "norm");
            } else {
                panic!("No right of right");
            }
        } else {
            panic!("No right");
        }
    }
}