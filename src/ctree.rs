use rand;
use rand::Rng;

pub struct CTree {
    pub branches: Vec<Branch>,
    pub max_width: usize,
} 

pub struct Branch {
    pub width: usize,
    pub ornaments: Vec<usize>,
}


impl CTree {
    pub fn new(height: usize) -> CTree {
        let h = if height > 4 { height } else { 4 };
        let mut branches: Vec<Branch> = Vec::with_capacity(h);

        // simple ornament distribution right now
        for i in 0..height {
            let w = (i*2)+1;

            let mut rng = rand::thread_rng();
            let mut ornaments = Vec::new();
            let mut ready = 0;

            let prev_branch_os = if i > 2 {
                branches[i-1].ornaments.clone()
            } else {
                Vec::new()
            };

            for j in 1..(w-1) {

                if ready == 0 && !prev_branch_os.contains(&(j-1)) && rng.gen() {
                    ornaments.push(j);
                    ready = 2;
                } else if ready > 0 {
                    ready -= 1;
                }
            }

            let branch = Branch {
                width: w,
                ornaments: ornaments
            };
            
            branches.push(branch);
        }

        let max_w = branches[branches.len()-1].width;
        CTree {
            branches: branches,
            max_width: max_w,
        }
    }
}
