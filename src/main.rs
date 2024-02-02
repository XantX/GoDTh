mod history;

use history::return_history;
use history::TreeNode;
use std::io::{self, Read, Write};

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn get_child(node: &TreeNode) -> Option<&TreeNode> {
    if node.childen.len() == 1 {
        Some(&node.childen[0])
    } else {
        None
    }
}


fn main() {
    let history_tree: history::TreeNode = return_history();
    let mut current_node = &history_tree;
    loop {
        clear_terminal();
        println!("{}", current_node.line);

        println!("\n\n\nPress 'n' to continue.");

        if let Some(input) = read_key() {
            if input == 'n' {
                if let Some(child_node) = get_child(&current_node) {
                    current_node = child_node;
                } else {
                    break;
                }
            }
        }
    }
    println!("Fin del juego.")

}

fn read_key() -> Option<char> {
    let mut buffer = [0; 1];
    match io::stdin().read_exact(&mut buffer) {
        Ok(_) => Some(buffer[0] as char),
        Err(_) => None,
    }
}

