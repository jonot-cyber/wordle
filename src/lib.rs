#![feature(portable_simd)]
use wasm_bindgen::prelude::*;
use rayon::prelude::*;
use std::simd::{cmp::SimdPartialEq, u8x8, u32x8};

pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
extern {
    pub fn alert(msg: &str);
}

fn get_num(guess: u8x8, correct: u8x8) -> u8x8 {
    let mut info: u8x8 = Default::default();

    let mut used = guess.simd_eq(correct);

    for i in 0..5 {
        if guess[i] != correct[i] {
            let find_set = correct.simd_eq(u8x8::splat(guess[i])) & !used;
            info[i] = if find_set.any() {
                used.set(find_set.first_set().unwrap(), true);
                1
            } else {
                2
            }
        }
    }
    info
}

pub fn matches_info(word: u8x8, guess: u8x8, info: u8x8) -> bool {
    let mut used = word.simd_eq(guess) & info.simd_eq(u8x8::splat(0));

    if (word.simd_ne(guess) & info.simd_eq(u8x8::splat(0))).any() {
        return false
    }

    if (word.simd_eq(guess) & info.simd_eq(u8x8::splat(1))).any() {
        return false
    }

    for i in 0..5 {
        let find_set = word.simd_eq(u8x8::splat(guess[i])) & !used;
        if find_set.any() {
            if info[i] == 2 {
                return false
            } else if info[i] == 1 {
                used.set(find_set.first_set().unwrap(), true)
            }
        } else if info[i] == 1 {
            return false
        }
    }
    true
}

pub fn guess_format(guess: &[u8]) -> u8x8 {
    let mut ret: u8x8 = Default::default();
    ret[0] = guess[0];
    ret[1] = guess[1];
    ret[2] = guess[2];
    ret[3] = guess[3];
    ret[4] = guess[4];
    ret
}

fn get_information(guess: u8x8, infos: &[u8x8], possible_words: &[u8x8]) -> f64 {
    let mut values: Vec<i32> = Vec::new();
    for info in infos {
        let count = possible_words
            .par_iter()
            .filter(|w| matches_info(**w, guess, *info))
            .count();
        values.push(count as i32);
    }
    let mut probs: Vec<f64> = Vec::new();
    for _ in 0..(3_i32.pow(5)) {
        probs.push(0.0)
    }
    for word in possible_words {
        let num: u32 = (get_num(guess, *word) * u8x8::from_array([81, 27, 9, 3, 1, 0, 0, 0])).as_array().iter().map(|w| *w as u32).sum();
        probs[num as usize] += 1.0
    }
    for i in 0..probs.len() {
        probs[i] /= possible_words.len() as f64;
    }

    let sum: f64 = values
        .iter()
        .zip(probs.iter())
        .map(|(v, p)| {
            f64::from(*v) * *p
        })
        .sum();

    -(sum / f64::from(possible_words.len() as u32)).log2()
}

fn numtoword(word: u8x8) -> String {
    String::from_utf8(word.to_array()[..5].to_vec()).unwrap()
}

#[wasm_bindgen]
pub fn get_best_word(js_guesses: Box<[String]>, js_info: Box<[u8]>) -> String {
    let word_file = include_str!("words.txt");
    let mut words: Vec<u8x8> = Vec::new();
    
    let mut i = 0;
    while i < word_file.len() {
        let section = word_file[i..i + 6].as_bytes();
        let word: u8x8 = guess_format(section);
        words.push(word);
        i += 6;
    }

    let mut possible_words = words.clone();
    let found_out = js_guesses.iter().zip(js_info.chunks(5));
    for f in found_out {
        possible_words = possible_words
            .par_iter()
            .filter(|w| matches_info(**w, guess_format(f.0.as_bytes()), guess_format(&f.1)))
            .map(|x| *x)
            .collect();
    }
    let mut infos: Vec<u8x8> = Vec::new();
    for i0 in 0..3 {
        for i1 in 0..3 {
            for i2 in 0..3 {
                for i3 in 0..3 {
                    for i4 in 0..3 {
                        infos.push(guess_format(&[i0, i1, i2, i3, i4]))
                    }
                }
            }
        }
    }

    let sums: Vec<f64> = words
        .par_iter()
        .map(|w| get_information(*w, &infos, &possible_words))
        .collect();
    
    let sums_boosted = sums
        .par_iter()
        .enumerate()
        .map(|(i, sum)| {
            sum + if possible_words.contains(&words[i]) {
                let frac = f64::from(possible_words.len() as i32) / f64::from(words.len() as i32);
                -frac.log2() / 16.0
            } else {
                0.0
            }
        });
    
    let (word, score) = words
        .par_iter()
        .zip(sums_boosted)
        .max_by(|(_, s), (_, s2)| {
            s.partial_cmp(s2).unwrap()
        }).unwrap();
    numtoword(*word)
}