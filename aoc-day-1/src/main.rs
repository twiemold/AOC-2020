use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn read_file() -> Result<Vec<u32>, io::Error> {
    let file = File::open("src/input.txt")?;
    let file = BufReader::new(file);
    let mut nums: Vec<u32> = Vec::new();

    for line in file.lines() {
        let new_int = line.unwrap().parse::<u32>().unwrap();
        nums.push(new_int);
    }
    // for num in &nums {
    //     println!("{}", num);
    // }

    Ok(nums)
}

fn find_sum(nums: Vec<u32>, target: u32) -> (u32, u32) {
    let mut nums_hash: HashMap<u32,u32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let diff = target - num;
        if nums_hash.contains_key(&diff) {
            return (nums[*nums_hash.get(&diff).unwrap() as usize], nums[i]);
        } else {
            nums_hash.insert(*num, i.try_into().unwrap());
        }
    }

    (0, 0)
}

fn find_three_sum(nums: &mut Vec<u32>, target: u32) -> Vec<Vec<u32>> {
    let mut answers: Vec<Vec<u32>> = Vec::new();
    nums.sort();

    for (i, a) in nums.iter().enumerate() {
        if i > 0 && a == &nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;
        
        while left < right {
            let three_sum = a + nums[left] + nums[right];
            if three_sum > target {
                right = right - 1;
            } else if three_sum < target {
                left = left + 1;
            } else {
                answers.push(vec![*a, nums[left], nums[right]]);
                return answers;
            }
        }
    }

    answers
}


fn main() {
    let mut nums = read_file().unwrap();
    let vals = find_three_sum(&mut nums, 2020);
    let answer = vals[0][0]*vals[0][1]*vals[0][2];
    println!("The answer is {}", answer);
}
