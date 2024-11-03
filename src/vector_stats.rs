use std::collections::HashMap;

pub fn main() {
    let mut v = vec!(2,4,5,6,7,4,3,2,7,7,8);
    v.sort();
    println!("vector: {:?}", v);
    println!("median: {}", v[v.len()/2]);

    let mut count_map = HashMap::<i32,i32>::new();
    for val in v {
        *count_map.entry(val).or_default() += 1;
    }
    let key_with_max_value = count_map.iter().max_by_key(|entry | entry.1).unwrap();
    println!("mode: {}", key_with_max_value.0);

}
