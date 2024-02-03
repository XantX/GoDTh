mod character;
mod history;

use history::return_history;
use history::TreeNode;
use std::io::BufRead;
use std::io::{self, Read, Write};

use crate::character::Kratos;

fn grant_weapon(kratos: &Kratos, weapons: &Vec<String>) -> Kratos {
    let mut new_kratos: Kratos = kratos.clone();
    for weapon in weapons {
        new_kratos.weapon.push(weapon.to_string());
    }
    return new_kratos;
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn get_child(node: &mut TreeNode) -> Option<&mut TreeNode> {
    if node.childen.len() == 1 {
        Some(&mut node.childen[0])
    } else {
        None
    }
}

fn show_desicions(node: &TreeNode) {
    for (_option, desicion) in node.decisions.iter() {
        println!("{desicion}");
    }
}

fn main() {
    let mut history_tree: history::TreeNode = return_history();
    let mut current_node = &mut history_tree;
    let next_line = "\n\n\nPress 'n' to continue.\nPress 'c' to show character info.";
    let option_line = "\n\n\nWrite the option to continue.";
    let mut is_option_moment: bool;
    let mut kratos = Kratos::new(Vec::new(), 10, Vec::new(), 100);
    loop {
        clear_terminal();
        println!("{}", current_node.line);
        if current_node.do_action {
            kratos = current_node.action(grant_weapon, &kratos);
        }
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
            if is_option_moment {
                let option_selected = match input.to_digit(10) {
                    Some(n) if n - 1 < current_node.childen.len() as u32 => n as usize,
                    _ => {
                        println!("Opción inválida.");
                        continue;
                    }
                };
                println!("{option_selected}");
                current_node = &mut current_node.childen[option_selected - 1];
            }

            if input == 'n' && !is_option_moment {
                if let Some(child_node) = get_child(current_node) {
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
            let stdin = io::stdin();
            let mut lock = stdin.lock();
            let _ = lock.read_until(b'\n', &mut Vec::new());
            Some(buffer[0] as char)
        }
        Err(_) => None,
    }
}

fn show_character_info(kratos: &Kratos) {
    println!("{:?}", kratos)
}
