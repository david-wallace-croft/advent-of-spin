use super::*;

#[test]
fn test1() {
  let candidates = make_candidates();
  assert_eq!(candidates.len(), 60);
}
