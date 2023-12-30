// Improvements from QuickUnion
// Keep track of the length of objects AKA group of connected nodes
// The root of smaller tree goes into root of bigger tree

// The run time for a find operation is at most log base 2 of N
// The run time for connected operation is also log base 2 of N

#[derive(Debug)]
pub struct WeightedQuickUnion {
    pub ids: Vec<u32>,
    pub sizes: Vec<u32>,
}

impl WeightedQuickUnion {
    pub fn new(len: u32) -> Self {
        Self {
            ids: (0..len).collect(),
            sizes: vec![1; len as usize],
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

    pub fn union(&mut self, p: u32, q: u32) {
        let root_p = self.root(p) as usize;
        let root_q = self.root(q) as usize;

        if root_q == root_p {
            return;
        };
        if &self.sizes[root_p] < &self.sizes[root_q] {
            self.ids[root_p] = root_q as u32;
            self.sizes[root_q] += self.sizes[root_p];
        } else {
            self.ids[root_q] = root_p as u32;
            self.sizes[root_p] += self.sizes[root_q];
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_weighted_quick_union() {
        let mut quick_union = WeightedQuickUnion::new(10);
        assert_eq!(quick_union.ids, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(quick_union.sizes, vec![1; 10]);

        let root_2 = quick_union.root(2);
        assert_eq!(root_2, 2);

        assert!(!quick_union.connected(1, 2));

        // Connect 2 --- 1
        quick_union.union(2, 1);

        let root_1 = quick_union.root(1);
        assert_eq!(root_1, 2);

        let root_2 = quick_union.root(2);
        assert_eq!(root_2, 2);

        assert!(quick_union.connected(1, 2));
        assert_eq!(quick_union.ids, vec![0, 2, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(quick_union.sizes, vec![1, 1, 2, 1, 1, 1, 1, 1, 1, 1]);

        // Connect 1 --- 0
        quick_union.union(1, 0);

        let root_0 = quick_union.root(0);
        assert_eq!(root_0, 2);

        let root_2 = quick_union.root(2);
        assert_eq!(root_2, 2);

        assert!(quick_union.connected(1, 2));
        assert!(quick_union.connected(0, 2));
        assert!(quick_union.connected(1, 0));

        // (0, 2) -> 1
        assert_eq!(quick_union.ids, vec![2, 2, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(quick_union.sizes, vec![1, 1, 3, 1, 1, 1, 1, 1, 1, 1]);
    }
}
