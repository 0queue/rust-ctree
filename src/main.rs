extern crate rand;
extern crate term_painter;
mod ctree;

use ctree::CTree;
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;
use rand::Rng;

fn main() {
    let t = CTree::new(10);
    print_ctree(&t);
}

fn print_ctree(t: &CTree) {
    let ocolors = vec![Red, Blue, Yellow];
    let mut rng = rand::thread_rng();

    for b in &t.branches {
        for _ in 0..((t.max_width-b.width)/2) {
            print!(" ");
        }

        let mut os = b.ornaments.clone();
        os.reverse();
        let mut cur_ornament = os.pop();

        for i in 0..(b.width) {
            match (i, cur_ornament) {
                (x, Some(o)) if x == o => {
                    let rc = ocolors[(rng.next_u32() as usize) % ocolors.len()];
                    print!("{}", Plain.bg(rc).paint(" "));
                    cur_ornament = os.pop();
                },
                _ => print!("{}", Plain.bg(Green).paint(" ")),
            }
        }

        println!("");
    }

    // print trunk
    for _ in 0..2 {
        for _ in 0..((t.max_width-3)/2) {
            print!(" ");
        }

        println!("{}", Plain.bg(Red).paint("   "));
    }
}
