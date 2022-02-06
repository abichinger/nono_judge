mod base;

fn main() {

    let mut desc = base::Description::new();
    desc.push((1,base::Block::COLOR(1)));
    desc.push((2,base::Block::COLOR(1)));

    let line = base::Line {
        blocks: vec![base::Block::UNKNOWN, base::Block::UNKNOWN, base::Block::UNKNOWN, base::Block::UNKNOWN,base::Block::UNKNOWN,base::Block::UNKNOWN],
        desc: desc
    };

    for x in base::LineIterator::from(line) {
        println!("{:?}", x);
    }

    println!("Hello, world!");
}
