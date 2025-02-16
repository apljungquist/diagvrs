use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fs;

macro_rules! map (
    { $($key:expr => $value:expr),+$(,)? } => {
        {
            let mut result = HashMap::new();
            $(
                result.insert($key, $value);
            )+
            result
        }
     };
);
use dot_parser::*;
use itertools::Itertools;
use diagvrs::orderings;

fn main() {
    let dot = fs::read_to_string("deps.dot").unwrap();

    let graph = canonical::Graph::from(ast::Graph::try_from(dot.as_str()).unwrap());

    let aliases = map! {
        "\"ms_graphql_lms::aurora::learning_activity\""=>"a:la",
        "\"ms_graphql_lms::aurora::role\""=>"a:r",
        "\"ms_graphql_lms::aurora::skill\""=>"a:s",
        "\"ms_graphql_lms::aurora::user_learning_activity\""=>"a:ula",
        "\"ms_graphql_lms::aurora::user_learning_activity::sql_helpers\""=>"a:ula:h",
        "\"ms_graphql_lms::aurora::user_role\""=>"a:ur",
        "\"ms_graphql_lms::aurora::user_skill\""=>"a:us",
        "\"ms_graphql_lms::graphql::shared::id_cursor\""=>"g:s:c",

        "\"ms_graphql_lms\"" => "/",
        "\"ms_graphql_lms::aurora::learning_activity::mutation\"" => "a:la:m",
        "\"ms_graphql_lms::aurora::learning_activity::query\"" => "a:la:q",
        "\"ms_graphql_lms::aurora::role::mutation\"" => "a:r:m",
        "\"ms_graphql_lms::aurora::role::query\"" => "a:r:q",
        "\"ms_graphql_lms::aurora::skill::mutation\"" => "a:s:m",
        "\"ms_graphql_lms::aurora::skill::query\"" => "a:s:q",
        "\"ms_graphql_lms::aurora::user_learning_activity::mutation\"" => "a:ula:m",
        "\"ms_graphql_lms::aurora::user_learning_activity::query\"" => "a:ula:q",
        "\"ms_graphql_lms::aurora::user_role::mutation\"" => "a:ur:m",
        "\"ms_graphql_lms::aurora::user_role::query\"" => "a:ur:q",
        "\"ms_graphql_lms::aurora::user_skill::mutation\"" => "a:us:m",
        "\"ms_graphql_lms::aurora::user_skill::query\"" => "a:us:q",
        "\"ms_graphql_lms::graphql::learning_activity::mutation\"" => "g:la:m",
        "\"ms_graphql_lms::graphql::learning_activity::query\"" => "g:la:q",
        "\"ms_graphql_lms::graphql::learning_activity::types\"" => "g:la:t",
        "\"ms_graphql_lms::graphql::mutation\"" => "g:m",
        "\"ms_graphql_lms::graphql::query\"" => "g:q",
        "\"ms_graphql_lms::graphql::role::mutation\"" => "g:r:m",
        "\"ms_graphql_lms::graphql::role::query\"" => "g:r:q",
        "\"ms_graphql_lms::graphql::role::types\"" => "g:r:t",
        "\"ms_graphql_lms::graphql::skill::mutation\"" => "g:s:m",
        "\"ms_graphql_lms::graphql::skill::query\"" => "g:s:q",
        "\"ms_graphql_lms::graphql::skill::types\"" => "g:s:t",
        "\"ms_graphql_lms::graphql::user_learning_activity::mutation\"" => "g:ula:m",
        "\"ms_graphql_lms::graphql::user_learning_activity::query\"" => "g:ula:q",
        "\"ms_graphql_lms::graphql::user_learning_activity::types\"" => "g:ula:t",
        "\"ms_graphql_lms::graphql::user_role::mutation\"" => "g:ur:m",
        "\"ms_graphql_lms::graphql::user_role::query\"" => "g:ur:q",
        "\"ms_graphql_lms::graphql::user_role::types\"" => "g:ur:t",
        "\"ms_graphql_lms::graphql::user_skill::mutation\"" => "g:us:m",
        "\"ms_graphql_lms::graphql::user_skill::query\"" => "g:us:q",
        "\"ms_graphql_lms::graphql::user_skill::types\"" => "g:us:t",
    };

    let a2i: HashMap<_, _> = aliases.values().into_iter().enumerate().map(|(i, v)| (v, i)).collect();
    let i2a: HashMap<_, _> = aliases.values().into_iter().enumerate().collect();

    let mut missing = HashSet::new();

    let mut order: Vec<&str> = Vec::new();
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    for edge in graph.edges.set {
        let Some(from) = &aliases.get(edge.from.as_str()) else {
            missing.insert(edge.from.as_str().to_string());
            continue;
        };
        let Some(to) = aliases.get(edge.to.as_str()) else {
            missing.insert(edge.to.as_str().to_string());
            continue;
        };
        if !order.contains(to) {
            order.push(to);
        }
        if !order.contains(from) {
            order.push(from);
        }
        tree.entry(to).or_default().push(from);
    }
    if !missing.is_empty() {
        for m in missing.into_iter().sorted().collect::<Vec<_>>() {
            println!("{m:?}");
        }
        return;
    }

    for k in &order {
        tree.entry(k).or_default();
    }

    let mut inv_tree:HashMap<&str, Vec<&str>> = HashMap::new();
    for (k,vs) in tree.iter() {
        inv_tree.entry(k).or_default();
        for v in vs {
            let entry = inv_tree.entry(v).or_default();
            if !entry.contains(&k) {
                entry.push(k);
            }

        }
    }

    let norm_tree: HashMap<_, _> = tree.iter().map(|(k, vs)| (a2i[k], vs.iter().map(|v| a2i[v]).collect_vec())).collect();
    for norm_order in orderings(&norm_tree).step_by(100_000).take(0) {
        let order = norm_order.into_iter().map(|i|*i2a[i]).collect_vec();
        println!("pseudo random order {order:?}");
        println!("{}", diagvrs::formatted(&tree, &order));
    }

    println!("Alphabetical order:");
    order.sort();
    println!("{}", diagvrs::formatted(&tree, &order));

    println!("Reverse alphabetical order:");
    order.sort_by_key(|k| std::cmp::Reverse(k.to_string()));
    println!("{}", diagvrs::formatted(&tree, &order));

    println!("Incoming edges > Reverse alphabetical order:");
    order.sort_by_key(|k| std::cmp::Reverse(k.to_string()));
    order.sort_by_key(|k| inv_tree[k].len());
    println!("{}", diagvrs::formatted(&tree, &order));

    println!("Incoming edges > Depth order:");
    order.sort_by_key(|k| std::cmp::Reverse(k.chars().filter(|c| *c == ':').count()));
    order.sort_by_key(|k| inv_tree[k].len());
    println!("{}", diagvrs::formatted(&tree, &order));

    println!("Depth > Incoming edges order:");
    order.sort_by_key(|k| inv_tree[k].len());
    order.sort_by_key(|k| std::cmp::Reverse(k.chars().filter(|c| *c == ':').count()));
    println!("{}", diagvrs::formatted(&tree, &order));

    println!("Depth > Reverse alphabetical order:");
    order.sort_by_key(|k| std::cmp::Reverse(k.to_string()));
    order.sort_by_key(|k| std::cmp::Reverse(k.chars().filter(|c| *c == ':').count()));
    println!("{}", diagvrs::formatted(&tree, &order));
}
