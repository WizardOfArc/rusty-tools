use std::collections::HashSet;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  items: String,
  number: usize,
}

#[derive(Debug)]
struct StackFrame {
    available: Vec<String>,
    chosen: Vec<String>,
    number_to_choose: usize,
}

fn get_combo_compliment(items: Vec<String>, combo: Vec<String>) -> Vec<String> {
    let mut new_combo: Vec<String> = Vec::new();
    let old_combo: HashSet<String> = HashSet::from_iter(combo);
    let mut index = 0;
    while index < items.len() {
        let item_to_check = &items[index];
        if !old_combo.contains(item_to_check) {
            new_combo.push(item_to_check.to_string());
        }
        index += 1;
    }
    new_combo
}

fn get_combos(items: Vec<String>, number: usize) -> HashSet<Vec<String>> {
    let mut the_stack: Vec<StackFrame> = Vec::new();
    let mut combo_list: HashSet<Vec<String>> = HashSet::new();
    the_stack.push(StackFrame{available: items.clone(), chosen: Vec::new(), number_to_choose: number});
    while !the_stack.is_empty() {
        match the_stack.pop() {
            None => (),
            Some(stack_frame) => {
                if stack_frame.number_to_choose == 0 || stack_frame.available.is_empty(){
                    combo_list.insert(stack_frame.chosen);
                } else {
                    for item in stack_frame.available.clone().into_iter() {
                        let mut others = stack_frame.available.clone();
                        let mut index = 0;
                        while index < others.len() {
                            if others[index] == item {
                                others.remove(index);
                            } else {
                              index += 1;
                            }
                        }
                        let mut new_chosen = stack_frame.chosen.clone();
                        new_chosen.push(item);
                        new_chosen.sort();
                        the_stack.push(StackFrame{available: others, chosen: new_chosen, number_to_choose: stack_frame.number_to_choose - 1})
                    }
                }
            }
        }
    }
    combo_list
}

fn main() {
    let args: Args = Args::parse();
    let item_list: Vec<String> = args.items.split(",").map(|s| s.trim().to_string()).collect();
    let mut combos: Vec<Vec<String>> = Vec::new();
    if args.number >= item_list.len() {
        combos.push(item_list)
    } else if args.number < (item_list.len() + 1) / 2 {
        combos = get_combos(item_list, args.number).into_iter().collect();
    } else {  // do the inverse
        let cloned_item_list = item_list.clone();
        let compliment_number = cloned_item_list.len() - args.number;
        let inverted_combos: Vec<Vec<String>> = get_combos(cloned_item_list, compliment_number).into_iter().collect();
        combos = inverted_combos.into_iter().map(|combination| get_combo_compliment(item_list.clone(), combination)).collect();
    }
    let total = combos.len();
    println!("Total combinations: {}.", total);
    for (idx, combo) in combos.into_iter().enumerate(){
        let for_printing = combo.join(", ");
        println!("{}: {}", idx + 1, for_printing);
    }

}
