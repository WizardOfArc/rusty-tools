
fn unique<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    list.sort_by(|a: &T, b: &T| a.cmp(b));
    list.dedup();
    list
}


fn main() {
   let inputted= vec![1,5,2,3,4,3,3,4,3,5,2,5];
   let expected = vec![1,2,3,4,5];
   assert_eq!(unique(inputted), expected);
}
