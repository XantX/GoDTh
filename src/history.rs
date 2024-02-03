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

    let mut line11 = new_node(11,
                          String::from("The weapons of chaos melted in the arms of kratos and with the swords he defeated the king of the barbarians."),
                          HashMap::new(),
                          Vec::new());

    let mut line12 = new_node(12,
                          String::from("From that day on, General Kratos became the emissary of destruction on behalf of Ares."),
                          HashMap::new(),
                          Vec::new());

    let mut line13 = new_node(13,
                          String::from("In the name of Ares, he killed innocents, plundered countless cities and destroyed empires."),
                          HashMap::new(),
                          Vec::new());

    let mut line14 = new_node(14,
                          String::from("One day Ares sent Kratos to destroy a temple dedicated to Athena."),
                          HashMap::new(),
                          Vec::new());

    let mut line15 = new_node(15,
                          String::from("As he stood in front of the temple, Kratos had a feeling. Something told him that he should not cross the gates of that temple."),
                          HashMap::new(),
                          Vec::new());

    let mut line16 = new_node(16,
                          String::from("The temple oracle warns you that the dangers inside the temple are greater than you can imagine."),
                          HashMap::from([
                            (0, String::from("1) Kratos enters the temple")),
                            (1, String::from("2) Kratos does not enter the temple"))
                          ]),
                          Vec::new());

    // First final: Kratos don't kill his family
    let line18 = new_node(18,
                          String::from("Kratos raises his gaze to the sky and sees Ares in the clouds."),
                          HashMap::new(),
                          Vec::new());

    let mut line17 = new_node(17,
                          String::from("Kratos enters the temple and kills every man, woman and child that crosses his path."),
                          HashMap::new(),
                          Vec::new());

    let mut line19 = new_node(19,
                          String::from("Not realizing that among the innocents were his wife Lysandra and daughter Kaliope."),
                          HashMap::new(),
                          Vec::new());

    let mut line20 = new_node(20,
                          String::from("Repentant for the act he has just committed, Kratos swears revenge against the god who has entrusted him with this task."),
                          HashMap::new(),
                          Vec::new());

    let line21 = new_node(21,
                          String::from("Upon leaving the temple, the Oracle of Athena cursed Kratos by painting his body with the ashes of his family."),
                          HashMap::new(),
                          Vec::new());

    line20.childen.push(line21);
    line19.childen.push(line20);
    line17.childen.push(line19);
    line16.childen.push(line17);
    line16.childen.push(line18);

    line15.childen.push(line16);
    line14.childen.push(line15);
    line13.childen.push(line14);
    line12.childen.push(line13);
    line11.childen.push(line12);
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
