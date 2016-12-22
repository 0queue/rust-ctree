extern crate rand;
extern crate term_painter;
mod ctree;

use ctree::CTree;
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;
use rand;
use rand::Rng;

fn main() {
    let t = CTree::new(10);
    print_ctree(t);
}

fn print_ctree(t: &CTree) {
    let ocolors = vec![Red, Blue, Yellow];

    for b in t.branches {
        for i in 0..(b.width-t.max_width) {
            print!(" ");
        }

        let mut os = b.ornaments.clone();
        os.reverse();
        let mut cur_ornament = os.pop();
        for i in 0..width {
            match (i, cur_ornament) {
                (_, None) => print!("{}", Plain.bg(Green).paint(" "));
                (i, Some(o)) where i == 0 => print
            }
        }
    }
}
