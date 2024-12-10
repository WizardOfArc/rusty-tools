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
    let item_list: Vec<String> = args.items.split("|").map(|s| s.trim().to_string()).collect();
    println!("Hello, You gave me these items:{:?}, and this number:{}", item_list, args.number);
    let mut combos: Vec<Vec<String>> = Vec::new();
    if args.number >= item_list.len() {
        combos.push(item_list)
    } else {
        combos = get_combos(item_list, args.number).into_iter().collect();
    }
    let mut lines: Vec<String>  = vec!["\\version \"2.24.4\"".to_string()];
    lines.push("{".to_string());
    lines.push("    d''1".to_string());

    for combo in combos.into_iter(){
        let for_printing = combo.join(" ");
        lines.push(format!("   <{}>", for_printing));
    }
    lines.push("}".to_string());
    std::fs::write("combo_out.ly", lines.join("\n")).unwrap();
}
