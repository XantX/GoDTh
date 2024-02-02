mod history;
mod character;

use history::return_history;
use history::TreeNode;
use std::io::BufRead;
use std::io::{self, Read, Write};

use crate::character::Kratos;

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

fn show_desicions(node: &TreeNode) {
    for desicion in &node.decisions {
        println!("{desicion}");
    }
}

fn main() {
    let history_tree: history::TreeNode = return_history();
    let mut current_node = &history_tree;
    let next_line = "\n\n\nPress 'n' to continue.";
    let option_line = "\n\n\nWrite the option to continue.";
    let mut is_option_moment: bool;
    let kratos = Kratos::new(Vec::new(), 10, Vec::new(),100);
    loop {
        clear_terminal();
        println!("{}", current_node.line);
        if current_node.decisions.len() > 1 {
            show_desicions(current_node);
            is_option_moment = true;
            println!("{option_line}")
        } else {
            is_option_moment = false;
            println!("{next_line}")
        }

        if current_node.childen.len() == 0 {
            break;
        }

        if let Some(input) = read_key() {
            if input == '1' && is_option_moment {
                current_node = &current_node.childen[0]
            }
            if input == '2' && is_option_moment {
                current_node = &current_node.childen[1]
            }
            if input == 'n' && !is_option_moment {
                if let Some(child_node) = get_child(&current_node) {
                    current_node = child_node;
                } else {
                    break;
                }
            }
            if input == 'c' && !is_option_moment {
                clear_terminal();
                show_character_info(&kratos);
                println!("To return to history enter any letter.");
                if let Some(input_character) = read_key() {
                    if input_character == 'b' && !is_option_moment {}
                }
            }
        }
    }
    println!("Fin del juego.")

}

fn read_key() -> Option<char> {
    let mut buffer = [0; 1];
    match io::stdin().read_exact(&mut buffer) {
        Ok(_) => {
            // Limpiar el buffer de entrada para evitar leer caracteres adicionales
            let stdin = io::stdin();
            let mut lock = stdin.lock();
            let _ = lock.read_until(b'\n', &mut Vec::new());
            Some(buffer[0] as char)
        },
        Err(_) => None,
    }
}

fn show_character_info(kratos: &Kratos) {
    println!("{:?}", kratos)
}
