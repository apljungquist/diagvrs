use crate::NormTree;
use std::collections::HashMap;
macro_rules! map (
    { $($key:expr => $value:expr),+ $(,)? } => {
        {
            let mut result = HashMap::new();
            $(
                result.insert($key, $value);
            )+
            result
        }
     };
);

pub fn diagv() -> NormTree {
    map! {
        0 => vec![2],
        1 => vec![2, 3],
        2 => vec![4],
        3 => vec![4]
    }
}

pub fn cyclic() -> NormTree {
    map! {
        0 => vec![1,4],
        1 => vec![],
        2 => vec![6],
        3 => vec![4],
        4 => vec![8],
        5 => vec![4],
        6 => vec![0],
        7 => vec![],
        8 => vec![2,7]
    }
}

pub fn cycle(n: usize) -> Result<NormTree, &'static str> {
    if n < 1 {
        return Err("Implemented only for 0<n");
    }
    let mut result = HashMap::new();
    for i in 0..n - 1 {
        result.insert(i, vec![i + 1]);
    }
    result.insert(n - 1, vec![0]);
    Ok(result)
}

pub fn sonic(n: usize) -> Result<HashMap<String, Vec<String>>, &'static str> {
    if !(1..=9).contains(&n) {
        return Err("Implemented only for 0<n<10");
    }
    let origin = String::from("x1");
    let mut result = HashMap::new();
    result.insert(origin.clone(), vec![]);
    for i in 2..n + 1 {
        result.get_mut(&origin).unwrap().push(format!("{}2", i));
        for j in 2..i {
            result.insert(format!("{}{}", i, j), vec![format!("{}{}", i, j + 1)]);
        }
        result.insert(format!("{}{}", i, i), vec![origin.clone()]);
    }
    Ok(result)
}
