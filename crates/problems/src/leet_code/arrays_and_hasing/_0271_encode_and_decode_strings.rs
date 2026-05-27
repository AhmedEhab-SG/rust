#[allow(unused_imports)]
use utils::Runable;

use crate::Solution;

// problem: https://leetcode.com/problems/encode-and-decode-strings/
impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        strs.iter()
            .map(|str| format!("{}#{}", str.len(), str))
            .collect()
    }

    pub fn decode(s: String) -> Vec<String> {
        // let mut result: Vec<String> = Vec::new();
        //
        // let mut curr = 0;
        //
        // while curr < s.len() {
        //     let start = curr;
        //
        //     while s.chars().nth(curr) != Some('#') {
        //         curr += 1
        //     }
        //
        //     let length = s
        //         .chars()
        //         .skip(start)
        //         .take(curr - start)
        //         .collect::<String>()
        //         .parse::<usize>()
        //         .unwrap();
        //
        //     curr += 1;
        //
        //     result.push(s.chars().skip(curr).take(length).collect::<String>());
        //
        //     curr += length;
        // }
        //
        // result

        // let mut res = Vec::new();
        // let bytes = s.as_bytes();
        //
        // let mut curr = 0;
        //
        // while curr < s.len() {
        //     let mut start = curr;
        //
        //     while bytes[start] != b'#' {
        //         start += 1;
        //     }
        //
        //     let length = std::str::from_utf8(&bytes[curr..start])
        //         .unwrap()
        //         .parse::<usize>()
        //         .unwrap();
        //
        //     start += 1;
        //
        //     let start_word = start;
        //     let end_word = start + length;
        //
        //     let word = str::from_utf8(&bytes[start_word..end_word])
        //         .unwrap()
        //         .to_string();
        //
        //     res.push(word);
        //
        //     curr = end_word
        // }
        //
        // res

        let mut res = Vec::new();
        let bytes = s.as_bytes();

        let mut curr_indx = 0;

        while curr_indx < bytes.len() {
            let hash_indx = bytes[curr_indx..].iter().position(|&c| c == b'#').unwrap() + curr_indx;
            // let hash_indx = s[curr_indx..].find('#').unwrap() + curr_indx;

            let length = s[curr_indx..hash_indx].parse::<usize>().unwrap();

            let start = hash_indx + 1;
            let end = start + length;

            res.push(s[start..end].to_string());

            curr_indx = end;
        }

        res
    }
}

impl Runable for Solution {
    fn run() {
        assert_eq!(
            Self::decode(Self::encode(vec!["Hello".to_string(), "World".to_string()])),
            vec!["Hello".to_string(), "World".to_string()]
        );

        assert_eq!(
            Self::decode(Self::encode(vec![
                "neet".to_string(),
                "code".to_string(),
                "love".to_string(),
                "you".to_string()
            ],)),
            vec![
                "neet".to_string(),
                "code".to_string(),
                "love".to_string(),
                "you".to_string()
            ]
        );

        assert_eq!(
            Self::decode(Self::encode(vec!["".to_string()])),
            vec!["".to_string()]
        );
    }
}
