use std::collections::HashMap;
use types::CowString;
use uuid::Uuid;

pub struct Store<I> {
    data: HashMap<String, I>,
}

impl<I> Store<I> {
    fn new() -> Self {
        Store {
            data: HashMap::new(),
        }
    }

    fn get<'store, S: Into<CowString>>(&'store self, key: S) -> Option<&'store I> {
        self.data.get(&key.into().into_owned())
    }

    fn update<'store, S: Into<CowString>>(&'store mut self, key: S, item: I) -> Option<I> {
        self.data.insert(key.into().into_owned(), item)
    }

    fn put<'store>(&'store mut self, item: I) -> String {
        let key = Uuid::new_v4().to_string();
        self.data.insert(key.clone(), item);
        key
    }

    fn remove<S: Into<CowString>>(&mut self, key: S) -> Option<I> {
        self.data.remove(key.into().as_ref())
    }

    fn find(&self, f: impl Fn(&I) -> bool) -> Option<&I> {
        for (_, value) in self.data.iter() {
            if f(value) == true {
                return Some(value);
            }
        }
        None
    }

    fn find_all(&self, f: impl Fn(&I) -> bool) -> Vec<&I> {
        let mut result: Vec<&I> = Vec::new();
        for (_, value) in self.data.iter() {
            if f(value) == true {
                result.push(value)
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Task {
        name: String,
        done: bool,
    }

    #[test]
    fn putting_and_getting_data() {
        let mut store: Store<Task> = Store::new();

        let task = Task {
            name: "A".to_string(),
            done: false,
        };

        let key = store.put(task.clone());

        assert_eq!(store.get(key), Some(&task));
    }

    #[test]
    fn removing_and_updating_data() {
        let mut store: Store<Task> = Store::new();

        let task_a = Task {
            name: "A".to_string(),
            done: false,
        };

        let task_b = Task {
            name: "B".to_string(),
            done: false,
        };

        let task_c = Task {
            name: "C".to_string(),
            done: false,
        };

        let key_a = store.put(task_a.clone());
        let key_b = store.put(task_b.clone());
        let key_c = store.put(task_c.clone());

        let task_updated = Task {
            name: "B".to_string(),
            done: true,
        };

        store.update(key_b.clone(), task_updated.clone());
        store.remove(key_c.clone());

        assert_eq!(store.get(key_a), Some(&task_a));
        assert_eq!(store.get(key_b), Some(&task_updated));
        assert_eq!(store.get(key_c), None);
    }

    #[test]
    fn finding_data() {
        let mut store: Store<Task> = Store::new();

        let task_a = Task {
            name: "A".to_string(),
            done: false,
        };

        let task_b = Task {
            name: "B".to_string(),
            done: true,
        };

        let task_c = Task {
            name: "C".to_string(),
            done: false,
        };

        let key_a = store.put(task_a.clone());
        let key_b = store.put(task_b.clone());
        let key_c = store.put(task_c.clone());

        let completed_task = store.find(|t| t.done == true);

        assert_eq!(store.get(key_b), completed_task);

        let undone_tasks = store.find_all(|t| t.done == false);

        assert!(undone_tasks.contains(&&task_a));
        assert!(undone_tasks.contains(&&task_c));
    }
}
