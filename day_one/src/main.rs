/*
    Day One - Advent of Code

*/

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

fn main() {
    println!("Advent of Code, 2024, Day 1, Puzzle 1!");

    let path = Path::new("data.txt");
    let file = File::open(&path).expect("File not found.");
    
    let reader = BufReader::new(&file);
    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();
    
    for line in reader.lines() {        
        let line = line.expect("Could not read file.");

        let ids = line.split_whitespace();
        
        let mut temp = Vec::<i32>::new();
        for id in ids {
            temp.push(id.parse().expect("Could not convert str to int."));
        }
        
        // not a fan of this.... better way to do all of this?
        if temp.len() == 2 {
            left_list.push(temp[0]);
            right_list.push(temp[1]);
        } 
    }

    // our lists should be the same size at this point
    assert!(left_list.len() == right_list.len());

    // sort vectors
    left_list.sort();
    right_list.sort();

    // both lists should now be sorted in ascending order
    // we now need to get the difference between each respective number and 
    // aggregate a sum
    let mut sum = 0;
    
    for i in 0..left_list.len() {
        let diff = left_list[i] - right_list[i];
        sum += diff.abs();
    }

    println!("Total distance: {}", sum);

    // part 2
    println!("Advent of Code, 2024, Day 1, Puzzle 2!");

    let mut similarity = 0;

    for i in 0..left_list.len() {
        let mut num_in_right_list = 0;
        for j in 0..right_list.len() {
            if left_list[i] == right_list[j] {
                num_in_right_list += 1;
            }
        }
        similarity += (left_list[i] * num_in_right_list);
    }

    println!("Total similarity: {}", similarity);

}
