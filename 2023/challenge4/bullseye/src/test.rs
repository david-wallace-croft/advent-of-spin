use super::*;

#[test]
fn test1() {
  let permutations = make_permutations();
  assert_eq!(permutations.len(), 60);
}
