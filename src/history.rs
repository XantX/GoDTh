use std::collections::HashMap;

use crate::character::Kratos;

#[derive(Debug)]
pub struct TreeNode {
    pub line: String,
    pub value: i32,
    pub childen: Vec<TreeNode>,
    pub decisions: HashMap<i32, String>,
    pub object_granted: Vec<String>,
    pub do_action: bool,
}

fn new_node(value: i32, line: String, decisions: HashMap<i32, String>, object_granted: Vec<String>) -> TreeNode {
    TreeNode { line, value, childen: Vec::new(), decisions, object_granted, do_action: false}
}

impl TreeNode {
    pub(crate) fn action(&mut self, action: fn(&Kratos, &Vec<String>) -> Kratos, kratos: &Kratos) -> Kratos {
        let weapons : Vec<String> = self.object_granted.clone();
        self.object_granted.clear();
        return action(kratos, &weapons);
    }
}

fn create_history() -> TreeNode {
    // Presentacion
    let mut root = new_node(0,
                            String::from("Fan-adapted story: XantX"),
                            HashMap::new(),
                            Vec::new());

    // History part 1 
    let mut line1 = new_node(1,
                             String::from("Long ago there was a Spartan general named kratos dedicated to the art of war and combat."),
                             HashMap::new(),
                             Vec::new());
    let mut line2 = new_node(2,
                             String::from("Wherever he went, he caused chaos and destruction to his enemies."),
                             HashMap::new(),
                             Vec::new());
    let mut line3 = new_node(3,
                             String::from("One day his army faced a tough opponent."),
                             HashMap::new(),
                             Vec::new());
    let mut line4 = new_node(4,
                             String::from("The barbarian army"),
                             HashMap::new(),
                             Vec::new());
    let mut line5 = new_node(5,
                             String::from("The Barbarons mercilessly ravaged the Spartan army."),
                             HashMap::new(),
                             Vec::new());
    let mut line6 = new_node(6,
                             String::from("Cornered, Kratos had to make a decision."),
                             HashMap::from([
                                (0, String::from("1) Invoke Ares, god of war")),
                                (1, String::from("2) Accept your destiny"))
                             ]),
                             Vec::new());

    // First end
    let line8 = new_node(8,
                         String::from("Kratos takes an axe from the Bavarian general and dies in battle"),
                         HashMap::new(),
                         Vec::new());


    // Second line
    let mut line7 = new_node(7,
                             String::from("Kratos shouts to the heavens the deal for Ares."),
                             HashMap::new(),
                             Vec::new());
    let mut line9 = new_node(9,
                             String::from("Kratos promises that if he gives him the power to defeat his enemies, he will be his servant for all eternity."),
                             HashMap::new(),
                             Vec::new());

    let mut line10 = new_node(10,
                          String::from("Ares came down from Olympus to defeat the barbarians and bestowed upon Kratos the 'Swords of Chaos' and melted into Kratos' arms."),
                          HashMap::new(),
                          vec![
                            String::from("Sword of caos")
                          ]);
    line10.do_action = true;
    let line11 = new_node(11,
                          String::from("Siguiente capitulo"),
                          HashMap::new(),
                          Vec::new());

    line10.childen.push(line11);
    line9.childen.push(line10);
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
