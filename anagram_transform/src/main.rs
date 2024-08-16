use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  word_a: String,
  word_b: String,
}


// make a function that gets a new word based on swapping two chars at given indexes
fn new_string_from_swap(word: &str, idx1: usize, idx2: usize) -> String {
    if idx2 >= word.len() {
        panic!("second index is out of bounds");
    }
    if idx1 > idx2 {
        panic!("first index overlaps second index");
    }
    let mut char_vec: Vec<char> = word.chars().collect();
    let swap_holder = char_vec[idx1];
    char_vec[idx1] = char_vec[idx2];
    char_vec[idx2] = swap_holder;
    char_vec.iter().collect::<String>()
}

fn get_swappable_indexes(word_a: &str, word_b: &str) -> Vec<usize> {
    let mut swappables: Vec<usize> = Vec::new();
    let length = word_a.len();
    let word_a_chars: Vec<char> = word_a.chars().collect();
    let word_b_chars: Vec<char> = word_b.chars().collect();
    for index in 0..length {
        if word_a_chars[index] != word_b_chars[index] {
            swappables.push(index);
        }
    }
    swappables
}


fn path_from_parent_mapping(mapping: &HashMap<String, Option<String>>, child: &String) -> Vec<String> {
    let mut path: Vec<String> = Vec::new();
    if !mapping.contains_key(child) {
        panic!("Something is wrong  - child is not in path");
    }
    path.push(child.clone());
    let mut current_word = mapping.get(child).unwrap();
    loop {
        match current_word {
            Some(word) => {
                path.push(word.clone());
                current_word = mapping.get(word).unwrap();
            },
            None => break,
        }
    }
    path.into_iter().rev().collect()
}

fn path_from_both_parent_mapping(forward_mapping: &HashMap<String, Option<String>>, backward_mapping: &HashMap<String, Option<String>>, child: &String) -> Vec<String> {
    let mut forward_path: Vec<String> = path_from_parent_mapping(forward_mapping, child);
    let mut reverse_path: Vec<String> = path_from_parent_mapping(backward_mapping, child);
    reverse_path.reverse();
    for word in reverse_path[1..].iter() {
        forward_path.push(word.to_string());
    }
    forward_path
}


fn find_shortest_path_bewteen_words(word_a: &str, word_b: &str) -> Vec<String> {
    let mut parent_mapping: HashMap<String, Option<String>> = HashMap::new();
    let mut reverse_parent_mapping: HashMap<String, Option<String>> = HashMap::new();
    let mut seen_words: HashSet<String> = HashSet::new();
    let mut reverse_seen_words: HashSet<String> = HashSet::new();
    let swappables = get_swappable_indexes(word_a, word_b);
    let mut word_queue: VecDeque<String> = VecDeque::new();
    let mut reverse_word_queue: VecDeque<String> = VecDeque::new();
    parent_mapping.insert(word_a.to_string(), None);
    reverse_parent_mapping.insert(word_b.to_string(), None);
    seen_words.insert(word_a.to_string());
    reverse_seen_words.insert(word_b.to_string());
    word_queue.push_back(word_a.to_string());
    reverse_word_queue.push_back(word_b.to_string());
    while word_queue.len() > 0 && reverse_word_queue.len() > 0 {
        let current_word = word_queue.pop_front().unwrap();
        if reverse_parent_mapping.contains_key(&current_word.clone()) {
            return path_from_both_parent_mapping(&parent_mapping, &reverse_parent_mapping, &current_word.clone());
        }
        let current_reverse_word = reverse_word_queue.pop_front().unwrap();
        if parent_mapping.contains_key(&current_reverse_word.clone()) {
            return path_from_both_parent_mapping(&reverse_parent_mapping, &parent_mapping, &current_reverse_word.clone());
        }
        for meta_idx_2 in 1..swappables.len() {
            for meta_idx_1 in 0..meta_idx_2 {
              let idx1 = swappables[meta_idx_1];
              let idx2 = swappables[meta_idx_2];
              let new_word = new_string_from_swap(&current_word.clone(), idx1, idx2);
              let reverse_new_word = new_string_from_swap(&current_reverse_word.clone(), idx1, idx2);
              if seen_words.contains(&new_word.clone()) {
                  continue;
              }
              if reverse_seen_words.contains(&reverse_new_word.clone()) {
                  continue;
              }
              reverse_seen_words.insert(reverse_new_word.clone());
              seen_words.insert(new_word.clone());
              if !reverse_parent_mapping.contains_key(&reverse_new_word.clone()){
                  reverse_parent_mapping.insert(reverse_new_word.clone(), Some(current_reverse_word.clone()));
                  reverse_word_queue.push_back(reverse_new_word.clone()); 
              }
              if !parent_mapping.contains_key(&new_word.clone()){
                  parent_mapping.insert(new_word.clone(), Some(current_word.clone()));
                  word_queue.push_back(new_word.clone());
              }
              if reverse_new_word == word_a.to_string() {
                  let mut path = path_from_parent_mapping(&reverse_parent_mapping, &reverse_new_word);
                  path.reverse();
                  return path;
              }
              if new_word == word_b.to_string() {
                  return path_from_parent_mapping(&parent_mapping, &new_word)
              }
            }
        }
    }
    return Vec::new();
}

fn main() {
    let args: Args = Args::parse();
    let original_word_a = args.word_a;
    let lowered_word_a = original_word_a.to_lowercase();
    let no_space_word_a = lowered_word_a.replace(" ", "");
    let original_word_b = args.word_b;
    let lowered_word_b = original_word_b.to_lowercase();
    let no_space_word_b = lowered_word_b.replace(" ", "");
    let word_a = no_space_word_a.as_str();
    let word_b = no_space_word_b.as_str();
    if word_a.len() != word_b.len() {
        println!("Not anagram 'cause not the same length");
        return;
    }   
    let mut first_char_map = HashMap::new();
    for ch in word_a.chars() {
        let counts = first_char_map.entry(ch).or_insert(0);
        *counts += 1;
    }
    
    let mut second_char_map = HashMap::new();
    for ch in word_b.chars() {
        let counts = second_char_map.entry(ch).or_insert(0);
        *counts += 1;
    }
    
    for k in first_char_map.keys() {
        match first_char_map.get(k) {
            Some(v1) => {
                match second_char_map.get(k) {
                    Some(v2) => {
                        if v1 != v2 {
                            println!("Not an anagram, for char {k}, {v1} != {v2}");
                            return;
                        }
                    },
                    None => {
                        println!("Not an anagram, missing key {k} in second word");
                        return;
                    }
                }
            },
            None => {
                println!("This shouldn't happen as I am calling get on a key that exists");
                return;
            }
        }
    }
    for k in second_char_map.keys() {
        match second_char_map.get(k) {
            Some(v1) => {
                match first_char_map.get(k) {
                    Some(v2) => {
                        if v1 != v2 {
                            println!("Not an anagram, for char {k}, {v1} != {v2}");
                            return;
                        }
                    },
                    None => {
                        println!("Not an anagram, missing key {k} in first word");
                        return;
                    }
                }
            },
            None => {
                println!("This shouldn't happen as I am calling get on a key that exists");
                return;
            }
        }
    }
    let path = find_shortest_path_bewteen_words(word_a, word_b);
    println!("A shortest path from '{}' to '{}' is", original_word_a, original_word_b);
    for step in path {
        println!("{}", step);
    }
}