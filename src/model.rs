use std::collections::HashMap;
use std::hash::Hash;

pub struct Model<T> {
    pub nodes: Vec<Node<T>>,
}

impl<T> Model<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(items: impl IntoIterator<Item = T>) -> Model<T> {
        let mut map = HashMap::<T, HashMap<T, u32>>::new();
        let mut last = None;

        for item in items {
            if let Some(last) = last.take() {
                let children = map.get_mut(&last).unwrap();
                if let Some(n) = children.get_mut(&item) {
                    *n += 1;
                } else {
                    children.insert(item.clone(), 1);
                }
            }

            if map.get(&item).is_none() {
                map.insert(item.clone(), HashMap::new());
            }

            last = Some(item);
        }

        Model {
            nodes: map
                .into_iter()
                .map(|(item, children)| Node {
                    item: item,
                    weights: children.values().cloned().collect(),
                    children: children.into_iter().map(|(item, _)| item).collect(),
                })
                .collect(),
        }
    }
}

pub struct Node<T> {
    pub item: T,
    pub children: Vec<T>,
    pub weights: Vec<u32>,
}
