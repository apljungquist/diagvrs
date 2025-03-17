use crate::core::Graph;

macro_rules! graph {
    [$($key:expr => [$($value:expr),* $(,)?]),+ $(,)?] => {
        Graph::from_heads(vec![$(($key, vec![$($value),*])),+])
    }
}

pub fn diagv() -> Graph<char> {
    graph![
        'd' => ['a'],
        'i' => ['a', 'g'],
        'a' => ['v'],
        'g' => ['v']
    ]
}

pub fn cyclic() -> Graph<i32> {
    graph![
        0 => [1,4],
        1 => [],
        2 => [6],
        3 => [4],
        4 => [8],
        5 => [4],
        6 => [0],
        7 => [],
        8 => [2,7],
    ]
}

pub fn cycle(n: usize) -> Graph<usize> {
    assert!(0 < n, "Implemented only for 0<n");
    let mut edges = Vec::new();
    for i in 0..n - 1 {
        edges.push((i, i + 1));
    }
    edges.push((n - 1, 0));
    Graph::from_edges(edges)
}

pub fn sonic(n: usize) -> Graph<String> {
    assert!((1..=9).contains(&n), "Implemented only for 0<n<10");
    let origin = "x1";
    let mut edges = Vec::new();
    for i in 2..n + 1 {
        edges.push((origin.to_string(), format!("{}2", i)));
        for j in 2..i {
            edges.push((format!("{}{}", i, j), format!("{}{}", i, j + 1)))
        }
        edges.push((format!("{}{}", i, i), origin.to_string()));
    }
    Graph::from_edges(edges)
}
