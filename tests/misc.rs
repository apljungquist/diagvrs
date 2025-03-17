use diagv::generators;

static DIAGV_TEXT: &str = "\
d-----+
    i-+---+
      +-a-|---+
          +-g-+
              +-v\
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
    let graph = generators::diagv();
    let actual = graph.ascii().unwrap();
    let expected = DIAGV_TEXT;
    assert_eq!(actual, expected);
}

#[test]
fn test_format_works_for_cyclic() {
    let graph = generators::cyclic();
    let actual = graph.ascii().unwrap();
    let expected = CYCLIC_TEXT;
    assert_eq!(actual, expected);
}
#[test]
#[should_panic(expected = "Omitting self loop")]
fn test_format_not_implemented_for_cycle_1() {
    let graph = generators::cycle(1);
    graph.ascii().unwrap();
}
#[test]
fn test_format_works_for_sonic_3() {
    let graph = generators::sonic(3);
    let actual = graph
        .ascii_with_order(&vec![
            &String::from("x1"),
            &String::from("22"),
            &String::from("32"),
            &String::from("33"),
        ])
        .unwrap();
    let expected = SONIC_3_TEXT;
    assert_eq!(actual, expected);
}
