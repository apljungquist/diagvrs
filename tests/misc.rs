use diagv::{formatted, generators, normalize};

static DIAGV_TEXT: &str = "\
0-----+
    1-+---+
      +-2-|---+
          +-3-+
              +-4\
";

static CYCLIC_TEXT: &str = "\
+-0-+-----------+
|   +-1         |
|       +-2-----|-------+
|       |     3-+       |
|       |       +-4-----|-------+
|       |       +-----5 |       |
+-------|---------------+-6     |
        |                   +-7 |
        +-------------------+---+-8\
";

static SONIC_3_TEXT: &str = "\
+-x1-+----+
+----+-22 |
|         +-32-+
+--------------+-33\
";

#[test]
fn test_format_works_for_diagv() {
    let mut graph = generators::diagv();
    normalize(&mut graph);
    let actual = formatted(&graph, &vec![0, 1, 2, 3, 4]);
    let expected = DIAGV_TEXT;
    assert_eq!(actual, expected);
}

#[test]
fn test_format_works_for_cyclic() {
    let mut graph = generators::cyclic();
    normalize(&mut graph);
    let actual = formatted(&graph, &vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    let expected = CYCLIC_TEXT;
    assert_eq!(actual, expected);
}
#[test]
#[should_panic(expected = "not implemented")]
fn test_format_not_implemented_for_cycle_1() {
    let graph = generators::cycle(1).unwrap();
    formatted(&graph, &vec![0]);
}
#[test]
fn test_format_works_for_sonic_3() {
    let graph = generators::sonic(3).unwrap();
    let actual = formatted(
        &graph,
        &vec![
            String::from("x1"),
            String::from("22"),
            String::from("32"),
            String::from("33"),
        ],
    );
    let expected = SONIC_3_TEXT;
    assert_eq!(actual, expected);
}
