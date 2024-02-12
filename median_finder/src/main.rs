

fn median(mut list: Vec<f32>) -> Option<f32> {
    if list.is_empty() {
        return None;
    }
    list.sort_by(|x: &f32, y: &f32| { x.partial_cmp(y).unwrap()});
    let length = list.len();
    match length % 2 {
        1 => Some(list[(length)/2]),
        0 => {
            let upper = list[length/2];
            let lower = list[length/2 -1];
            Some( (upper + lower) / 2.0)
        },
        _ => None,
    }
}

fn main() {
    let list = vec![1.0,7.6,9.0,4.32, 10.0];
    println!("{:?}", median(list));
}
