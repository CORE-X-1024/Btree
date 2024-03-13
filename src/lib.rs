use  std::collections::VecDeque;
use std::os::fd::AsRawFd;

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
        match self.edges.len() {
            //ingen barn
            0 => {
                //self sort
                let len_key = self.keys.len();
                if self.keys.is_empty() {
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

                    //sjekk antall keys i barna
                    match self.edges[0].keys.len() {
                        3 => {
                            //flytt median opp
                            self.keys.insert(0, self.edges[0].keys.remove(1).unwrap());
                            // og Splitt node
                            let key_0 = self.edges[0].keys.pop_front().unwrap();
                            self.edges.insert(0, Node::new());
                            self.edges[0].keys.push_front(key_0);
                            //insert key
                            self.edges[0].insert_key(key);
                        },
                        _ => {
                            self.edges[0].insert_key(key);
                        },
                    }
                } else {

                    //sjekk antall keys i barna
                    match self.edges[1].keys.len() {
                        3 => {
                            //flytt median opp
                            self.keys.push_back(self.edges[1].keys.remove(1).unwrap());
                            // og Splitt node
                            let key_1 = self.edges[1].keys.pop_back().unwrap();
                            self.edges.push_back(Node::new());
                            self.edges[2].keys.push_front(key_1);
                            //insert key
                            self.edges[2].insert_key(key);
                        },
                        _ => {
                            self.edges[1].insert_key(key);
                        },
                    }
                }
            },
            //Tre barn
            3 => {
                //Finn hvilket egde vi skal gå
                let key_len = self.keys.len();
                for i in 0..key_len {

                    if key < self.keys[i] {
                        //Hvis barnen har max keys //Max keys --> splits and move up
                        if self.edges[i].keys.len() == 3 {
                            //Flytt median opp til keys, front.
                            self.keys.insert(i, self.edges[i].keys.remove(1).unwrap());
                            //refere til en ny egde
                            let key_0 = self.edges[i].keys.pop_front().unwrap();
                            self.edges.insert(i, Node::new());
                            self.edges[i].insert_key(key_0);
                            //innsett ny key etter splitt
                            self.edges[i].insert_key(key);
                            return;

                        } else {

                            self.edges[i].insert_key(key);
                            return;
                        }
                    }
                }

                //Hvis keys er høyest
                if key > self.keys[1] {
                    //Sjekk om barn har maks keys
                    if self.edges[key_len].keys.len() == 3 {

                        self.keys.push_back(self.edges[key_len].keys.remove(1).unwrap());
                        let key_2 = self.edges[key_len].keys.pop_back().unwrap();

                        self.edges.push_back( Node::new());
                        self.edges[key_len+1].insert_key(key_2);

                        self.edges[key_len+1].insert_key(key);
                    } else {
                        self.edges[key_len].insert_key(key);
                    }
                }
            }

            //todo 4 barn
            _ => panic!("{}", self.edges.len()),
        }
    }
}


impl BTree{

    fn new() -> BTree{ BTree { root : Node::new() } }

    pub fn add(&mut self, key : i32){

        if self.root.keys.len()< 3 {

            self.root.insert_key(key);

        } else {

            match self.root.edges.len() {
                0 => {
                    let mut child0 = Node::new();
                    let mut child1 = Node::new();

                    child0.keys.push_back(self.root.keys.pop_front().unwrap());
                    child1.keys.push_back(self.root.keys.pop_back().unwrap());

                    self.root.edges.push_back(child0);
                    self.root.edges.push_back(child1);

                    self.root.insert_key(key);
                },
                4 => {

                    //Opprett 2 barn
                    let mut child0 = Node::new();
                    let mut child1 = Node::new();

                    //flytte keys til nye barn.
                    child0.keys.push_back(self.root.keys.pop_front().unwrap());
                    child1.keys.push_back(self.root.keys.pop_back().unwrap());

                    //flytte barn fra root til barn som deres barn.
                    child0.edges.push_back(self.root.edges.pop_front().unwrap());
                    child0.edges.push_back(self.root.edges.pop_front().unwrap());
                    child1.edges.push_back(self.root.edges.pop_front().unwrap());
                    child1.edges.push_back(self.root.edges.pop_front().unwrap());

                    self.root.edges.push_back(child0);
                    self.root.edges.push_back(child1);

                    self.root.insert_key(key);

                }
                //todo 2,3  barn
                _ => panic!("{}",self.root.edges.len()),
            }
        }
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let mut tree = BTree::new();
        for i in 1..=52  {
            tree.add(i);
        }

        println!("{:?}", tree.root.keys);
        let len = tree.root.edges.len();
        for i in 0..len  {
            println!("Egde {}: {:?}", i, tree.root.edges[i].keys);

        }
        println!("test treet");
        println!("{:?}", tree.root.edges[0]);
        println!("{:?}", tree.root.edges[1]);




    }
}
