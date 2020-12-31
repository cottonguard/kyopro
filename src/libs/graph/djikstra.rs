use super::LabeledGraph;

pub fn djikstra<T>(g: &LabeledGraph<T>, s: usize) -> Vec<Option<T>>
where
    T: std::ops::Add<Output = T> + Ord + Default + Copy,
{
    use std::{cmp::Ordering, collections::BinaryHeap};

    struct Node<T>(usize, T);
    impl<T: PartialEq> PartialEq for Node<T> {
        fn eq(&self, other: &Self) -> bool {
            self.1 == other.1
        }
    }
    impl<T: Eq> Eq for Node<T> {}
    impl<T: PartialOrd> PartialOrd for Node<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            other.1.partial_cmp(&self.1)
        }
    }
    impl<T: Ord> Ord for Node<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            other.1.cmp(&self.1)
        }
    }

    let mut dist = vec![None; g.len()];
    dist[s] = Some(T::default());
    let mut que = BinaryHeap::new();
    que.push(Node(s, T::default()));
    while let Some(Node(u, d)) = que.pop() {
        if dist[u].map(|du| d > du).unwrap_or(false) {
            continue;
        }
        for &(v, w) in &g[u] {
            let dsuv = d + w;
            if dist[v].map(|dv| dsuv < dv).unwrap_or(true) {
                dist[v] = Some(dsuv);
                que.push(Node(v, dsuv));
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use crate::{graph::LabeledGraph, random::Pcg};
    #[test]
    fn djikstra() {
        const N: usize = 13;
        const M: usize = 29;
        let mut rand = Pcg::new(819);

        for _ in 0..10 {
            let mut g = LabeledGraph::builder(N);
            for _ in 0..M {
                let u = rand.next_u32() as usize % N;
                let v = rand.next_u32() as usize % N;
                let w = rand.next_u32() >> 10;
                g.edge(u, v, w);
            }
            let g = g.build();

            let s = rand.next_u32() as usize % N;

            let mut dist_bf = vec![None; N];
            dist_bf[s] = Some(0);
            for _ in 0..N {
                for u in 0..N {
                    if let Some(du) = dist_bf[u] {
                        for &(v, w) in &g[u] {
                            if let Some(dv) = dist_bf[v] {
                                if du + w < dv {
                                    dist_bf[v] = Some(du + w);
                                }
                            } else {
                                dist_bf[v] = Some(du + w);
                            }
                        }
                    }
                }
            }

            let dist_d = super::djikstra(&g, s);

            assert_eq!(dist_d, dist_bf);
        }
    }
}
