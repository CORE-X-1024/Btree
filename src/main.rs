use untitled::BTree;


fn main() {
    let mut tree = BTree::new();
    for i in 1..= 10000 {
        tree.add(i)
    }

    println!("{:?}", tree.root.keys);
}