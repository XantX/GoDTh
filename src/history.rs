#[derive(Debug)]
pub struct TreeNode {
    pub line: String,
    pub value: i32,
    pub childen: Vec<TreeNode>,
}

fn new_node(value: i32, line: String) -> TreeNode {
    TreeNode { line, value, childen: Vec::new()}
}

fn create_history() -> TreeNode {
    // Presentacion
    let mut root = new_node(0, String::from("Fan-adapted story: XantX"));

    // History part 1 
    let mut line1 = new_node(1, String::from("Long ago there was a Spartan general named kratos dedicated to the art of war and combat."));
    let mut line2 = new_node(2, String::from("Wherever he went, he caused chaos and destruction to his enemies."));
    let mut line3 = new_node(3, String::from("One day his army faced a tough opponent."));
    let mut line4 = new_node(4, String::from("The barbarian army"));
    let line5 = new_node(5, String::from("The Barbarons mercilessly ravaged the Spartan army."));

    line4.childen.push(line5);
    line3.childen.push(line4);
    line2.childen.push(line3);
    line1.childen.push(line2);
    root.childen.push(line1);

    return root;
}

pub fn return_history() -> TreeNode {
    return create_history()
}
