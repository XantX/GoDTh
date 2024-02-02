#[derive(Debug)]
pub struct TreeNode {
    pub line: String,
    pub value: i32,
    pub childen: Vec<TreeNode>,
    pub decisions: Vec<String>,
}

fn new_node(value: i32, line: String, decisions: Vec<String>) -> TreeNode {
    TreeNode { line, value, childen: Vec::new(), decisions}
}

fn create_history() -> TreeNode {
    // Presentacion
    let mut root = new_node(0, String::from("Fan-adapted story: XantX"), Vec::new());

    // History part 1 
    let mut line1 = new_node(1, String::from("Long ago there was a Spartan general named kratos dedicated to the art of war and combat."), Vec::new());
    let mut line2 = new_node(2, String::from("Wherever he went, he caused chaos and destruction to his enemies."), Vec::new());
    let mut line3 = new_node(3, String::from("One day his army faced a tough opponent."), Vec::new());
    let mut line4 = new_node(4, String::from("The barbarian army"), Vec::new());
    let mut line5 = new_node(5, String::from("The Barbarons mercilessly ravaged the Spartan army."), Vec::new());
    let mut line6 = new_node(6,
                         String::from("Cornered, Kratos had to make a decision."),
                         vec![
                            String::from("1) Invoke Ares, god of war"),
                            String::from("2) Accept your destiny")
                         ]);

    // First end
    let line8 = new_node(8, String::from("Kratos takes an axe from the Bavarian general and dies in battle"), Vec::new());

    let mut line7 = new_node(7, String::from("Kratos shouts to the heavens the deal for Ares."), Vec::new());

    let line9 = new_node(9, String::from("Kratos promises that if he gives him the power to defeat his enemies, he will be his servant for all eternity."), Vec::new());

    line7.childen.push(line9);
    line6.childen.push(line7);
    line6.childen.push(line8);
    line5.childen.push(line6);
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
