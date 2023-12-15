#[derive(Debug)]
pub struct QuickUnion {
    pub ids: Vec<u32>
}

impl QuickUnion {
    pub fn new(len: u32) -> Self {
        Self {
            ids: (0..len).collect()
        }
    }

    pub fn root(&self, id: u32) -> u32 {
        let mut id = id;
        while id != self.ids[id as usize] {
            id = self.ids[id as usize];
        }

        id
    }

    pub fn connected(&self, p: u32, q: u32) -> bool {
        self.root(p) == self.root(q)
    }

    pub fn union(&mut self, p: u32, q: u32)  {
        let root_p = self.root(p);
        let root_q = self.root(q);

        self.ids[root_p as usize] = root_q;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_quick_union() {
        let mut quick_union = QuickUnion::new(10);
        assert_eq!(quick_union.ids, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let root_2 = quick_union.root(2);
        assert_eq!(root_2, 2);

        assert!(!quick_union.connected(1, 2));

        // Connect 2 --- 1
        quick_union.union(2, 1);

        let root_2 = quick_union.root(2);
        assert_eq!(root_2, 1);

        assert!(quick_union.connected(1, 2));
        assert_eq!(quick_union.ids, vec![0, 1, 1, 3, 4, 5, 6, 7, 8, 9]);

        // Connect 1 --- 0
        quick_union.union(1, 0);

        let root_1 = quick_union.root(1);
        assert_eq!(root_1, 0);

        let root_2 = quick_union.root(2);
        assert_eq!(root_2, 0);

        assert!(quick_union.connected(1, 2));
        assert!(quick_union.connected(0, 2));
        assert!(quick_union.connected(1, 0));

        // 1 -> 0
        // 2 -> 1 -> 0
        assert_eq!(quick_union.ids, vec![0, 0, 1, 3, 4, 5, 6, 7, 8, 9]);
    }
}