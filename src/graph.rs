use crate::prelude::*;

pub trait Graph<Vertex, Edge>
where
    Vertex: std::hash::Hash + Clone + Copy + PartialEq + Eq + std::fmt::Debug,
{
    fn edges(&self, v: Vertex) -> impl IntoIterator<Item = Edge>;
    fn edge(&self, v: Vertex, e: Edge) -> (Vertex, u64);

    fn dijkstra_full(&self, start: Vertex) -> HashMap<Vertex, (Vertex, u64)> {
        let mut to_visit = priority_queue::PriorityQueue::<
            _,
            _,
            ahash::RandomState,
        >::with_default_hasher();
        let mut prev = HashMap::new();
        prev.insert(start, (start, 0));
        to_visit.push(start, std::cmp::Reverse(0));
        while let Some((v, std::cmp::Reverse(distance))) = to_visit.pop() {
            for e in self.edges(v) {
                let (next, weight) = self.edge(v, e);
                let visited = prev.contains_key(&next);
                let new_distance = distance + weight;
                if to_visit.get(&next).is_some() {
                    prev.insert(next, (v, new_distance));
                    if new_distance < to_visit.get_priority(&next).unwrap().0
                    {
                        to_visit.change_priority(
                            &next,
                            std::cmp::Reverse(new_distance),
                        );
                    }
                } else {
                    if !visited {
                        prev.insert(next, (v, new_distance));
                        to_visit.push(next, std::cmp::Reverse(new_distance));
                    }
                }
            }
        }
        prev
    }

    fn dijkstra<F: Fn(Vertex) -> bool>(
        &self,
        start: Vertex,
        end: F,
    ) -> Option<(u64, Vec<Vertex>)> {
        let mut to_visit = priority_queue::PriorityQueue::<
            _,
            _,
            ahash::RandomState,
        >::with_default_hasher();
        let mut prev = HashMap::new();
        prev.insert(start, start);
        to_visit.push(start, std::cmp::Reverse(0));
        while let Some((v, std::cmp::Reverse(distance))) = to_visit.pop() {
            if end(v) {
                let mut path = vec![v];
                let mut cur = v;
                while let Some(next) = prev.get(&cur) {
                    if *next == cur {
                        break;
                    }
                    path.insert(0, *next);
                    cur = *next;
                }
                return Some((distance, path));
            }

            for e in self.edges(v) {
                let (next, weight) = self.edge(v, e);
                let visited = prev.contains_key(&next);
                let new_distance = distance + weight;
                if to_visit.get(&next).is_some() {
                    prev.insert(next, v);
                    if new_distance < to_visit.get_priority(&next).unwrap().0
                    {
                        to_visit.change_priority(
                            &next,
                            std::cmp::Reverse(new_distance),
                        );
                    }
                } else {
                    if !visited {
                        prev.insert(next, v);
                        to_visit.push(next, std::cmp::Reverse(new_distance));
                    }
                }
            }
        }
        None
    }
}
