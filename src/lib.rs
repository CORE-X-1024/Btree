use std::collections::hash_map::Keys;
use  std::collections::VecDeque;

#[derive(Debug)]
pub struct Node{
    pub keys: VecDeque<i32>,
    edges: VecDeque<Node>,
}
pub struct BTree{ pub root : Node }

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
                if len_key == 0{
                    self.keys.push_back(key);
                } else {
                    for i in 0..len_key {
                        if key < self.keys[i] {
                            self.keys.insert(i, key);
                            return;
                        }
                    }
                    if key > self.keys[len_key - 1] {
                        self.keys.push_back(key);
                    }
                }
            },
            _ => {
                //sjekk enhverbarn sin keys.
                let len_key = self.keys.len();
                for i in 0..len_key {

                    if key < self.keys[i] {
                        //sjekk antall keys i barna
                        match self.edges[i].keys.len() {
                            3 => {
                                //todo sjekk edge igjen
                                if self.edges[i].edges.len() == 4{
                                    //Flytt opp median
                                    self.keys.insert(i, self.edges[i].keys.remove(1).unwrap());
                                    //Flytt noden med høyeste key med 2 barn for å bli en barn
                                    let mut node = Node::new();
                                    node.keys.push_back(self.edges[i].keys.pop_back().unwrap());
                                    node.edges.push_back(self.edges[i].edges.pop_back().unwrap());
                                    node.edges.push_front(self.edges[i].edges.pop_back().unwrap());

                                    self.edges.push_back(node);
                                    self.insert_key(key);
                                    return;


                                } else if self.edges[i].edges.is_empty() {

                                    //flytt median opp
                                    self.keys.insert(i, self.edges[i].keys.remove(1).unwrap());
                                    // og Splitt node
                                    let key_0 = self.edges[i].keys.pop_front().unwrap();
                                    self.edges.insert(i, Node::new());
                                    self.edges[i].keys.push_front(key_0);
                                    //insert key
                                    self.edges[i].insert_key(key);
                                    return;
                                }
                            },
                            _ => {
                                self.edges[i].insert_key(key);
                                return;
                            },
                        }
                    }

                    if key > self.keys[len_key - 1]{

                        match self.edges[len_key].keys.len() {
                            3 => {

                                if self.edges[len_key].edges.len() == 4{
                                    //Flytt opp median
                                    self.keys.insert(len_key, self.edges[len_key].keys.remove(1).unwrap());
                                    //Flytt noden med høyeste key med 2 barn for å bli en barn
                                    let mut node = Node::new();
                                    node.keys.push_back(self.edges[len_key].keys.pop_back().unwrap());
                                    node.edges.push_back(self.edges[len_key].edges.pop_back().unwrap());
                                    node.edges.push_front(self.edges[len_key].edges.pop_back().unwrap());

                                    self.edges.push_back(node);
                                    self.insert_key(key);
                                    return;


                                } else {
                                    //flytt median opp
                                    self.keys.push_back(self.edges[len_key].keys.remove(1).unwrap());
                                    // og Splitt node
                                    let key_1 = self.edges[len_key].keys.pop_back().unwrap();
                                    self.edges.insert(len_key + 1, Node::new());
                                    self.edges[len_key + 1].insert_key(key_1);
                                    //insert key
                                    self.edges[len_key + 1].insert_key(key);
                                }
                            },
                            _ => {
                                self.edges[len_key].insert_key(key);
                            },
                        }
                    }
                }
            }
        }
    }
    fn contain(&self, key : i32) -> bool {

        let len = self.keys.len();
        for i in 0..len  {

            if key < self.keys[i] {
                if self.edges.is_empty() {
                    return false
                } else { return self.edges[i].contain(key);}

            } else if key == self.keys[i] {
                return true;
            }
        }

        if key > self.keys[len - 1] {

            if self.edges.is_empty() {
                return false
            } else {
                return self.edges[len].contain(key);
            }
        }
        false
    }

}


impl BTree{

    pub fn new() -> BTree{ BTree { root : Node::new() } }

    pub fn add(&mut self, key : i32){

        self.root.insert_key(key);

        if self.root.keys.len() == 3{
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
                _ => panic!("Antall barn som må håndteres {}", self.root.edges.len()),
            }
        }
    }

    pub fn find(&self, key : i32) {
        let is_present = self.root.contain(key);

        if is_present {
            println!("Key {} Ok.", key);
        } else {
            println!("Key {} None.", key);
        }
    }

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {

        let mut tree = BTree::new();
        for i in 1..= 10000 {
            tree.add(i)
        }

        println!("{:?}", tree.root.keys);
        println!("{:?}", tree.find(100000));


        //let len = tree.root.edges.len();
        //for i in 0..len  {
        //    println!("leaves{}: {:?}  ", i, tree.root.edges[i].keys);
        //    let len2 = tree.root.edges[i].edges.len();
        //    for j in 0..len2  {
        //        print!("leaves{}.leefs{}: {:?}  ", i, j, tree.root.edges[i].edges[j].keys);
        //        let len3 = tree.root.edges[i].edges[j].edges.len();
        //        for k in 0..len3  {
        //            println!("");
        //            print!("leaves{}.leefs{}.more{}: {:?}   ", i, j, k, tree.root.edges[i].edges[j].edges[k].keys);
        //            let len4 = tree.root.edges[i].edges[j].edges[k].edges.len();
        //            for m in 0..len4  {
        //                println!("");
        //                print!("leaves{}.leefs{}.more{}.more{}: {:?}   ", i, j, k, m, tree.root.edges[i].edges[j].edges[k].edges[m].keys);
        //            }
        //        }
        //    }
        //}
    }

}
