


use  std::collections::VecDeque;

#[derive(Debug)]
pub struct Node{
    keys: VecDeque<i32>,
    edges: VecDeque<Node>,
}
pub struct BTree{ root : Node }

impl Node{

    fn new() -> Node{
        Node{
            keys : VecDeque::new(),
            edges: VecDeque::new(),
        }
    }

    fn insert_key(&mut self, key : i32){

        //Sjekk om det finnes edges
        let len = self.edges.len();

        match len {
            0 => {
                //self sort
                let len_key = self.keys.len();
                if self.keys.is_empty(){
                    self.keys.push_back(key);
                } else {
                    for i in 0..len_key {
                        if key < self.keys[i] {
                            self.keys.insert(i, key);
                            break;
                        }
                    }
                    if key > self.keys[len_key - 1] {
                        self.keys.push_back(key);
                    }
                }
            },
            //To barn
            2 => {
                //sjekk enhverbarn sin keys.
                if key < self.keys[0] {

                    let barn_keys_len = self.edges[0].keys.len();
                    //sjekk antall keys i barna
                    match barn_keys_len {
                        3 => {
                            //flytt median opp
                            self.keys.insert(0, self.edges[0].keys.remove(1).unwrap());
                            // og Splitt node
                            let key_0 = self.edges[0].keys.pop_front().unwrap();
                            self.edges.push_front(Node::new());
                            self.edges[0].keys.push_front(key_0);
                            //insert key
                            self.edges[0].insert_key(key);
                        },
                        _ => {
                            self.edges[0].insert_key(key);
                        },
                    }
                } else {

                    let barn_keys_len = self.edges[1].keys.len();
                    //sjekk antall keys i barna
                    match barn_keys_len {
                        3 => {
                            //flytt median opp
                            self.keys.push_back(self.edges[1].keys.remove(1).unwrap());
                            // og Splitt node
                            let key_1 = self.edges[1].keys.pop_back().unwrap();
                            self.edges.push_back(Node::new());
                            self.edges[1].keys.push_front(key_1);
                            //insert key
                            self.edges[1].insert_key(key);
                        },
                        _ => {
                            self.edges[1].insert_key(key);
                        },
                    }

                }
            },
            _ => (),
        }


    }
}


impl BTree{

    fn new() -> BTree{
        BTree { root : Node::new() }
    }

    pub fn add(&mut self, key : i32){


        let len_keys = self.root.keys.len();
        if len_keys < 3 {

            self.root.insert_key(key);

        } else {

            let len_e = self.root.edges.len();
            match len_e {
                0 => {
                    let mut child0 = Node::new();
                    let mut child1 = Node::new();

                    child0.keys.push_back(self.root.keys.pop_front().unwrap());
                    child1.keys.push_back(self.root.keys.pop_back().unwrap());

                    self.root.edges.push_back(child0);
                    self.root.edges.push_back(child1);

                    self.root.insert_key(key);
                },
                _ => println!("ll"),
            }
        }

            //splitt node
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let mut tree = BTree::new();
        for i in (0..=5).rev()  {
            tree.add(i);
        }

        println!("{:?}", tree.root);


    }
}
