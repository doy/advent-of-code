use crate::prelude::*;

pub trait Graph<Vertex, Edge>
where
    Vertex: std::hash::Hash + Clone + Copy + PartialEq + Eq + std::fmt::Debug,
{
    type Edges: IntoIterator<Item = Edge>;

    fn edges(&self, v: Vertex) -> Self::Edges;
    fn edge(&self, v: Vertex, e: Edge) -> (Vertex, i64);

    fn dijkstra(&self, start: Vertex, end: Vertex) -> (i64, Vec<Vertex>) {
        let mut to_visit = priority_queue::PriorityQueue::new();
        let mut prev = HashMap::new();
        prev.insert(start, start);
        to_visit.push(start, std::cmp::Reverse(0));
        while let Some((v, std::cmp::Reverse(distance))) = to_visit.pop() {
            if v == end {
                let mut path = vec![v];
                let mut cur = v;
                while let Some(next) = prev.get(&cur) {
                    if *next == cur {
                        break;
                    }
                    path.insert(0, *next);
                    cur = *next;
                }
                return (distance, path);
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
        unreachable!()
    }
}
