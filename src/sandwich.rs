use itertools::Itertools;
use std::collections::BTreeMap;

pub fn main() {
    gen_sandwich();
}

#[derive(Debug)]
struct Sandwich {
    pub sum: i32,
    pub combinations: Vec<Combination>,
}

#[derive(Debug)]
struct Combination {
    pub combination: Vec<i32>,
    pub permutations: Vec<Vec<i32>>,
}

impl Sandwich {
    pub fn gen_excel(&self) {
        println!("\nSandwich {}", self.sum);
        for combo in &self.combinations {
            println!("\t");
        }
    }
}

impl Combination {
    pub fn len(&self) {
        self.combination.len();
    }
}

fn gen_sandwich() {
    let mut sandwiches: BTreeMap<i32, Sandwich> = BTreeMap::new();
    let values = [2, 3, 4, 5, 6, 7, 8];
    for n in 1..=values.len() {
        for combo in values.iter().combinations(n) {
            //bg!(&combo);
            let mut sum = 0;
            for v in combo.iter() {
                sum += **v;
            }
            if !sandwiches.contains_key(&sum) {
                sandwiches.insert(sum, Sandwich {
                    sum,
                    combinations: vec![],
                });
            }
            let mut sandwich = sandwiches.get_mut(&sum).unwrap();
            let mut combination = Combination {
                combination: combo.iter().map( | x| **x).collect(),
                permutations: vec![],
            };
            // let sum: u8 = combo.iter().sum();
            //bg!(sum);
            for permutation in combo.iter().permutations(n) {
                // bg!(permutation);
                &combination.permutations.push(permutation.iter().map(|x| ***x).collect());
            }
            sandwich.combinations.push(combination);
        }
    }
    //bg!(&sandwiches);

    show_counts(&sandwiches);

    for sandwich in sandwiches.values() {
        sandwich.gen_excel();
    }

}

fn show_counts(sandwiches: &BTreeMap<i32, Sandwich>) {
    dbg!(sandwiches.len());
    let mut combo_sum = 0;
    let mut perm_sum = 0;
    for sandwich in sandwiches.values() {
        for combo in sandwich.combinations.iter() {
            combo_sum += 1;
            perm_sum += combo.permutations.len();
        }
    }
    dbg!(combo_sum);
    dbg!(perm_sum);
}
