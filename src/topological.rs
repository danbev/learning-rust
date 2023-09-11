use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Value<'a> {
    label: String,
    children: Vec<&'a Value<'a>>,
}

fn topological_sort<'a>(
    root: &'a Value<'a>,
    visited: &mut HashSet<&'a Value<'a>>,
    stack: &mut VecDeque<&'a Value<'a>>,
) {
    visited.insert(root);

    for child in root.children.iter() {
        if !visited.contains(child) {
            topological_sort(child, visited, stack);
        }
    }

    stack.push_front(root);
}

fn topological_order<'a>(value: &'a Value<'a>) -> VecDeque<&'a Value<'a>> {
    let mut visited = HashSet::new();
    let mut stack = VecDeque::new();
    topological_sort(&value, &mut visited, &mut stack);
    stack
}

fn main() {
    let three = Value {
        label: "3".to_string(),
        children: vec![],
    };
    let one = Value {
        label: "1".to_string(),
        children: vec![&three],
    };
    let two = Value {
        label: "2".to_string(),
        children: vec![&three],
    };
    let zero = Value {
        label: "0".to_string(),
        children: vec![&one, &two],
    };

    let stack = topological_order(&zero);

    println!("Topologically sorted order:");
    for value in stack {
        println!("value: {}", value.label);
    }
}
