
extern crate associative_positional_list;
use associative_positional_list::AssociativePositionalList;

#[test]
fn test_traits() {
    let mut p: AssociativePositionalList<String> = AssociativePositionalList::new();
    p.insert(0, "Hello".to_string());
    p.insert(1, "World".to_string());
    assert_eq!(p.len(), 2);
    assert_eq!(p[0], "Hello");
    assert_eq!(p[1], "World");
    //assert_eq!(p, ["Hello", "World"]);
}
