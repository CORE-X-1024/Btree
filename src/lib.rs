use std::collections::hash_map::Keys;
use  std::collections::VecDeque;

#[derive(Debug)]
pub struct Node{
    pub keys: VecDeque<i32>,
    edges: VecDeque<Node>,
}
pub struct BTree{ pub root : Node }

impl Node {
    fn new() -> Node {
        Node {
            keys: VecDeque::new(),
            edges: VecDeque::new(),
        }
    }

    fn insert_key(&mut self, key: i32) {

        //Sjekk om det finnes edges
        match self.edges.len() {
            //ingen barn
            0 => {
                //self sort
                let len_key = self.keys.len();
                if len_key == 0 {
                    self.keys.push_back(key);
                    return;
                } else {
                    for i in 0..len_key {
                        if key < self.keys[i] {
                            self.keys.insert(i, key);
                            return;
                        }
                    }
                    if key > self.keys[len_key - 1] {
                        self.keys.push_back(key);
                        return;
                    }
                }
            },
            _ => {
                //sjekk enhverbarn sin keys.
                let len_key = self.keys.len();
                //TODO HARDCODE THIS LOOP
                for i in 0..len_key {
                    if key < self.keys[i] {
                        //sjekk antall keys i barna
                        match self.edges[i].keys.len() {
                            3 => {
                                //todo sjekk edge igjen
                                if self.edges[i].edges.len() == 4 {
                                    //Flytt opp median
                                    self.keys.insert(i, self.edges[i].keys.remove(1).unwrap());
                                    //Flytt noden med høyeste key med 2 barn for å bli en barn
                                    let mut node = Node::new();
                                    node.keys.insert(i, self.edges[i].keys.pop_front().unwrap());

                                    node.edges.insert(i,self.edges[i].edges.pop_front().unwrap());
                                    node.edges.insert(i+1, self.edges[i].edges.pop_front().unwrap());
//
                                    self.edges.insert(i,node);
                                    self.insert_key(key);


                                } else {
//
                                    //flytt median opp
                                    self.keys.insert(i, self.edges[i].keys.remove(1).unwrap());
                                    // og Splitt node
                                    let key_0 = self.edges[i].keys.pop_front().unwrap();
                                    self.edges.insert(i, Node::new());
                                    self.edges[i].keys.push_front(key_0);
                                    //insert key
                                    self.insert_key(key);
                                    //println!("0-{}", key);
                                    break
                                }
                            },
                            _ => {
                                self.edges[i].insert_key(key);
                                //println!("1-{}", key);
                                return;
                            },
                        }
                    }
                }

                    if key > self.keys[len_key - 1] {
                        match self.edges[len_key].keys.len() {
                            3 => {
                                if self.edges[len_key].edges.len() == 4 {
                                    //Flytt opp median
                                    self.keys.insert(len_key, self.edges[len_key].keys.remove(1).unwrap());
                                    //Flytt noden med høyeste key med 2 barn for å bli en barn
                                    let mut node = Node::new();
                                    node.keys.push_back(self.edges[len_key].keys.pop_back().unwrap());
                                    node.edges.push_back(self.edges[len_key].edges.pop_back().unwrap());
                                    node.edges.push_front(self.edges[len_key].edges.pop_back().unwrap());

                                    self.edges.push_back(node);
                                    self.insert_key(key);
                                    //println!("2-{}", key);
                                    return;
                                } else {
                                    //flytt median opp
                                    self.keys.push_back(self.edges[len_key].keys.remove(1).unwrap());
                                    // og Splitt node
                                    let key_1 = self.edges[len_key].keys.pop_back().unwrap();
                                    self.edges.insert(len_key + 1, Node::new());
                                    self.edges[len_key + 1].insert_key(key_1);
                                    //insert key
                                    self.insert_key(key);
                                    //println!("3-{}", key);
                                }
                            },
                            _ => {
                                self.edges[len_key].insert_key(key);
                                //println!("4-{}", key);
                            },
                        }
                    }
                }
            }
        }


    fn contain(&self, key: i32) -> bool  {
        let len = self.keys.len();
        for i in 0..len {
            if key < self.keys[i] {
                if self.edges.is_empty() {
                    return false
                } else { return self.edges[i].contain(key); }
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

    fn print_keys(&self, liste : &mut Vec<i32>) {
        let key_len = self.keys.len();
        let mut i = 0;

        for egde in &self.edges {
            if egde.edges.is_empty() {
                for key in &egde.keys {
                    print!("{} ", key);
                    liste.push(key.clone());
                }
            } else {
                egde.print_keys(liste);
            }

            if i < key_len {
                print!("{} ", self.keys[i]);
                liste.push(self.keys[i]);
            }
            i += 1;
        }
    }
}


impl BTree{

    pub fn new() -> BTree{ BTree { root : Node::new() } }

    pub fn add(&mut self, key : i32){

        self.root.insert_key(key);

        let root_len = self.root.keys.len();
        if root_len == 3{
            match self.root.edges.len() {
                0 => {
                    let mut child0 = Node::new();
                    let mut child1 = Node::new();

                    child0.keys.push_back(self.root.keys.pop_front().unwrap());
                    child1.keys.push_back(self.root.keys.pop_back().unwrap());

                    self.root.edges.push_back(child0);
                    self.root.edges.push_back(child1);
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
                }
                _ => panic!("Antall barn som må håndteres {}", self.root.edges.len()),
            }


        } else if root_len == 2 && !self.root.edges.is_empty(){
            //TODO FIX LOGIKK HER

            //sjekk om barn er fult
            for m in 0..root_len{
                if self.root.edges[m].keys.len() == 3 {

                    let lenX = self.root.edges[m].edges.len();
                    match lenX {
                        0 => {
                            let mut child0 = Node::new();
                            let mut child1 = Node::new();

                            child0.keys.push_back(self.root.edges[m].keys.pop_front().unwrap());
                            child1.keys.push_back(self.root.edges[m].keys.pop_back().unwrap());

                            self.root.edges[m].edges.push_back(child0);
                            self.root.edges[m].edges.push_back(child1);
                            return;
                        },
                        4 => {

                            //Opprett 2 barn
                            let mut child0 = Node::new();
                            let mut child1 = Node::new();

                            //flytte keys til nye barn.
                            child0.keys.push_back(self.root.edges[m].keys.pop_front().unwrap());
                            child1.keys.push_back(self.root.edges[m].keys.pop_back().unwrap());

                            //flytte barn fra root til barn som deres barn.
                            child0.edges.push_back(self.root.edges[m].edges.pop_front().unwrap());
                            child0.edges.push_back(self.root.edges[m].edges.pop_front().unwrap());
                            child1.edges.push_back(self.root.edges[m].edges.pop_front().unwrap());
                            child1.edges.push_back(self.root.edges[m].edges.pop_front().unwrap());

                            self.root.edges[m].edges.push_back(child0);
                            self.root.edges[m].edges.push_back(child1);
                            return;

                        }
                        _ => panic!("Antall barn som må håndteres {}", self.root.edges[m].edges.len()),

                    }
                }
            } return;

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
    fn insert_rev() {
        let mut tree = BTree::new();
        let numbers = [
            12, 8, 28, 3, 21, 19, 17, 25, 24, 26,
            14, 4, 22, 11, 27, 13, 2, 10, 18, 15,
            20, 1, 6, 9, 5, 23, 30, 7, 29, 16, 31,
        ];
        for i in (0..40).rev() {
            tree.add(i)
        }




        //println!("{:?}", tree.root.keys);
        //let len = tree.root.edges.len();
        //for i in 0..len {
        //    println!("leaves{}: {:?}  ", i, tree.root.edges[i].keys);
        //    let len2 = tree.root.edges[i].edges.len();
        //    for j in 0..len2 {
        //        print!("leaves{}.leefs{}: {:?}  ", i, j, tree.root.edges[i].edges[j].keys);
        //        let len3 = tree.root.edges[i].edges[j].edges.len();
        //        for k in 0..len3 {
        //            println!("");
        //            print!("leaves{}.leefs{}.more{}: {:?}   ", i, j, k, tree.root.edges[i].edges[j].edges[k].keys);
        //            let len4 = tree.root.edges[i].edges[j].edges[k].edges.len();
        //            for m in 0..len4 {
        //                println!("");
        //                print!("leaves{}.leefs{}.more{}.more{}: {:?}   ", i, j, k, m, tree.root.edges[i].edges[j].edges[k].edges[m].keys);
        //            }
        //        }
        //    }
        //}
    }

    #[test]
    fn insert_in_ascending_order() {
        let mut tree = BTree::new();
        let mut list = Vec::new();
        let mut vec: Vec<i32> = (1..=40).collect();
        for i in 1..=40 {
            tree.add(i);
        }

        tree.root.print_keys(&mut list);
        assert_eq!(list, vec);

    }

    #[test]
    fn insert_in_descending_order() {
        let mut tree = BTree::new();
        let mut list = Vec::new();
        let mut vec: Vec<i32> = (0..40).collect();
        for i in (0..40).rev() {
            tree.add(i);
        }

        tree.root.print_keys(&mut list);
        assert_eq!(list, vec);
    }

    #[test]
    fn insert_in_random_order() {
        let mut tree = BTree::new();
        let numbers = [
            12, 8, 28, 3, 21, 19, 17, 25, 24, 26,
            14, 4, 22, 11, 27, 13, 2, 10, 18, 15,
            20, 1, 6, 9, 5, 23, 30, 7, 29, 16, 31,
        ];
        for &i in numbers.iter() {
            tree.add(i);
        }
        assert_eq!(tree.root.contain(20), true);
        assert_eq!(tree.root.contain(40), false);
    }

    #[test]
    fn insert_duplicate_values() {
        let mut tree = BTree::new();
        for _ in 0..40 {
            tree.add(10);
        }
        assert_eq!(tree.root.contain(10), true);
        assert_eq!(tree.root.contain(20), false);
    }

}
