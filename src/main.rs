use std::collections::HashMap;

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

fn main() {
    let tree = map! {
        "D" => vec!["A"],
        "I" => vec!["A", "G"],
        "A" => vec!["V"],
        "G" => vec!["V"],
        "V" => vec![],
    };
    let order = vec!["D", "I", "A", "G", "V"];
    println!("{}", diagvrs::formatted(&tree, &order));
}
