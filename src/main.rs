use usvg::NodeExt;

fn main() {
    let file = include_str!("../bug-bb.svg");
    
    let options = usvg::Options::default();
    let svg = usvg::Tree::from_str(file, &options.to_ref()).unwrap();
    
    let bbox = svg.root().calculate_bbox().unwrap();
    
    let mult = svg.svg_node().size.width() / svg.svg_node().view_box.rect.width();
    println!("{}", (bbox.x() - svg.svg_node().view_box.rect.x()) * mult);
    println!("{}", (bbox.y() - svg.svg_node().view_box.rect.y()) * mult);
    println!("{}", bbox.width() * mult);
    println!("{}", bbox.height() * mult);
    
    // println!("{}", bbox.x());
    // println!("{}", bbox.y());
    // println!("{}", bbox.width());
    // println!("{}", bbox.height());
}
