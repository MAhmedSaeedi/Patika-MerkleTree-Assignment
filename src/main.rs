/*
import Crates
*/
use std::fs::File;
use std::io::{prelude::*, BufReader};
use sha2::{Sha256, Digest};

fn merkle_root(filename: String) -> String {
    // Read Input Data from txt file
   
    
    let file = File::open(filename).expect("Error in reading file");
    let buf_reader = BufReader::new(file);
    
    // Create vector of strings for leaves
    
    let mut leaves_count :u64 = 0;
    let mut leaves:Vec<_> = Vec::new();
    for line in buf_reader.lines() {
        let line_as_str = line.expect("Cant read new line");
        if leaves_count == 0 {
            let level_count = line_as_str.parse::<u32>().unwrap();
            leaves_count = u64::pow(2,level_count);
            leaves = Vec::with_capacity(leaves_count as usize);
        }
        else{
            leaves.push(line_as_str.to_string());
        }
    }
    
    //If leave count is not a power of two, round leaves to the nearest power of two by duplicating leave before last end dynamically
    while leaves.len() < leaves_count as usize && leaves.len() >= 2 {
        leaves.push(leaves[leaves.len()-2].clone());
    }
    
    while leaves.len() > leaves_count as usize {
        leaves.remove(leaves.len()-1);
    }


    // Hash inputs and append to vector
    let leave_hashes = leaves.iter().map(|leave| hash_input(leave)).collect::<Vec<String>>();


    // Then Write an algorithm that calculates the ROOT
    let root = find_root_hash(leave_hashes);


    // Return the root hash as a String
    return root;


}

//Finds root hash by iterating through nodes on last created levels and creating next level with them.
fn find_root_hash(leave_hashes: Vec::<String>) -> String{
    let mut last_level = leave_hashes.clone();
    while last_level.len() != 1 {
        last_level = create_next_level(last_level);
    }
    return last_level[0].clone();
}

// You can use templates below or just remove
// Helper function to create a next leaves level may help you :)
fn create_next_level(current_level: Vec::<String>) -> Vec::<String> {
    let node_count = current_level.len();
    if node_count == 1 {
        return current_level;
    }

    //Next level have Fixed number of nodes 
    let node_count_on_new_level = node_count/2;
    let mut next_level_nodes = Vec::with_capacity(node_count_on_new_level);

    //Concatenate and add hash string of result to nodes (node#1 and node#2, node#2 and node#3, node#4 and node#5...)
    for i in 0..node_count_on_new_level {
        let hash_concat = format!("{}{}", current_level[i*2], current_level[i*2+1]);
        next_level_nodes.push(hash_input(hash_concat.as_str()));
    }
    return next_level_nodes;
}


// Helper function may help you to hash an input or You can write macro rules
fn hash_input(a: &str) -> String {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(a);

    // read hash digest and consume hasher
    return format!("{:x}", hasher.finalize());
}

fn main() { 
    merkle_root("input0.txt".to_string());
}



// Pass all tests!
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_0() {
        let result = merkle_root("input0.txt".to_string());
        assert_eq!(result, "ff41418be11ed77612aeb83ee0bcf97a5853a4c291e23bd4d4cc6435fcfabdf9");
    }

    #[test]
    fn input_1() {
        let result = merkle_root("input1.txt".to_string());
        assert_eq!(result, "98a77b2c3ff5f6c2aca697f60b2aa2a1a2733be36dbd35bae23d517c2ad5985e");
    }

    #[test]
    fn input_2() {
        let result = merkle_root("input2.txt".to_string());
        assert_eq!(result, "3c0fb0638de91551eae4e9d984d72034aa0693be37b51737e6b81bc489866e5e");
    }

    #[test]
    fn input_3() {
        let result = merkle_root("input3.txt".to_string());
        assert_eq!(result, "f03b1c9163babeb728ac011fe0c2c9c69700a2f8ddde211ec07d621cdb322cfe");
    }

    #[test]
    fn input_4() {
        let result = merkle_root("input4.txt".to_string());
        assert_eq!(result, "f83e74742fda659dfc07615881af796abafc434f591aeb23b9f4366abe03c597");
    }
}
