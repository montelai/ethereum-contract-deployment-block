use std::env;
use std::collections::HashMap;

fn main() {

    const usageStr: &str = "Usage:
        eth_contract_block <address> [--start <block number: int>] 
    ";

    let args: Vec<String> = env::args().collect();


    let mut env: HashMap<String, Vec<String>> = HashMap::new();

    //find indexes that contain --
    let vec_of_flag_positions: Vec<(usize, bool)> = args.iter()
        .filter_map(|value| Some(value.contains("--")))
        .enumerate()
        .filter(|tuple| tuple.1 == true)
        .collect();

    
    vec_of_flag_positions
        .windows(2)
        .for_each(|x| {
            env.insert(args[x[0].0].clone(), args[(x[0].0 + 1)..(x[1].0)].to_vec());
        });
    
    //get the remainder if its an uneven array
    if vec_of_flag_positions.len() % 2 != 0 {
        let last = &vec_of_flag_positions.last().unwrap();
        env.insert(args[last.0].clone(), args[(last.0 + 1)..].to_vec());
    }
}
