extern crate rand;
extern crate term_painter;
extern crate getopts;
mod ctree;

use ctree::CTree;
use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;
use rand::Rng;
use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("h", "height", "The height of a tree, larger than 8", "HEIGHT");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { println!("{}", f.to_string()); return; }
    };

    let height = match matches.opt_str("h") {
        Some(s) => s.parse().unwrap(),
        None => ctree::MIN_HEIGHT
    };

    let t = CTree::new(height);
    print_ctree(&t);
}

fn print_ctree(t: &CTree) {
    let ocolors = vec![Red, Blue, White];
    let mut rng = rand::thread_rng();

    for _ in 0..((t.max_width-1)/2) {
       print!(" ");
    }
    println!("{}", Plain.bg(Yellow).paint(" "));

    for _ in 0..((t.max_width-3)/2) {
       print!(" ");
    }
    println!("{}", Plain.bg(Yellow).paint("   "));

    for _ in 0..((t.max_width-1)/2) {
       print!(" ");
    }
    println!("{}", Plain.bg(Yellow).paint(" "));

    for b in &t.branches {
        if b.width == 1 { continue; }
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
