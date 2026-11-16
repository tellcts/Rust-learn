//! 串联所有单词的子串

use std::collections::HashMap;

/// [灵茶山艾府](https://leetcode.cn/problems/substring-with-concatenation-of-all-words/solutions/3691292/30-ci-ding-chang-hua-dong-chuang-kou-pyt-5vgx/)
pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<usize> {
        let word_len = words[0].len();
        let window_len = word_len * words.len();
        let mut target_cnt = HashMap::new();

        for w in &words {
            *target_cnt.entry(w.as_str()).or_insert(0) += 1;
        }

        let mut ans = vec![];

        for start in 0..word_len {
            let mut cnt = HashMap::new();
            let mut overload = 0;

            for right in (start + word_len..=s.len()).step_by(word_len) {
                let in_word = &s[right - word_len..right];
                let e = cnt.entry(in_word).or_insert(0);

                if e == target_cnt.get(in_word).unwrap_or(&0) {
                    overload += 1;
                }

                *e += 1;

                if right < window_len {
                    continue;
                }

                let left = right - window_len;

                if overload == 0 {
                    ans.push(left);
                }

                let out_word = &s[left..left + word_len];
                let e = cnt.entry(out_word).or_insert(0);
                *e -= 1;

                if e == target_cnt.get(out_word).unwrap_or(&0) {
                    overload -= 1;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn convert(values: Vec<&str>) -> Vec<String> {
        values.into_iter().map(String::from).collect()
    }

    #[rstest]
    #[case(String::from("barfoothefoobarman"),convert(vec!["foo","bar"]),vec![0,9])]
    #[case(String::from("barfoofoobarthefoobarman"),convert(vec!["bar","foo","the"]),vec![6,9,12])]
    #[case(String::from("wordgoodgoodgoodbestword"),convert(vec!["word","good","best","word"]),vec![])]
    fn test_find_substring(
        #[case] s: String,
        #[case] words: Vec<String>,
        #[case] expected: Vec<usize>,
    ) {
        assert_eq!(Solution::find_substring(s, words), expected);
    }
}
