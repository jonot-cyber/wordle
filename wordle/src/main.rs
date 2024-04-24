use std::{fs::File, io::Read};
use rayon::prelude::*;

fn buf_i(num: u64, i: u8) -> u8 {
    ((num >> i * 8) & 0xFF) as u8
}

fn get_num(guess: u64, correct: u64) -> u64 {
    let mut info = 0;
    for i in 0..5 {
        info *= 3;
        if buf_i(guess, i) != buf_i(correct, i) {
            let mut found = false;
            for j in 0..5 {
                if buf_i(guess, i) == buf_i(correct, j) {
                    found = true;
                }
            }
            info += if found {
                1
            } else {
                2
            }
        }
    }
    info
}

fn matches_info(word: u64, guess: u64, info: u64) -> bool {
    for i in 0..5 {
        if buf_i(info, i) == 0 {
            if buf_i(word, i) != buf_i(guess, i) {
                return false;
            }
        } else if buf_i(info, i) == 1 {
            if buf_i(word, i) == buf_i(guess, i) {
                return false;
            }
            let mut found = false;
            for j in 0..5 {
                if buf_i(guess, i) == buf_i(word, j) {
                    found = true
                }
            }
            if !found {
                return false;
            }
        } else {
            let mut found = false;
            for j in 0..5 {
                if buf_i(guess, i) == buf_i(word, j) {
                    found = true
                }
            }
            if found {
                return false;
            }
        }
    }
    true
}

fn guess_format(guess: &[u8]) -> u64 {
    (guess[0] as u64)
    + ((guess[1] as u64) << 8)
    + ((guess[2] as u64) << 16)
    + ((guess[3] as u64) << 24)
    + ((guess[4] as u64) << 32)
}

fn get_information(guess: u64, infos: &[u64], possible_words: &[u64]) -> f64 {
    let mut values: Vec<i32> = Vec::new();
    for info in infos {
        let count = possible_words
            .iter()
            .filter(|w| matches_info(**w, guess, *info))
            .count();
        values.push(count as i32);
    }
    let mut probs: Vec<f64> = Vec::new();
    for _ in 0..(3_i32.pow(5)) {
        probs.push(0.0)
    }
    for word in possible_words {
        probs[get_num(guess, *word) as usize] += 1.0
    }
    for i in 0..probs.len() {
        probs[i] /= possible_words.len() as f64;
    }

    let sum: f64 = values.iter().zip(probs.iter()).map(|(v, p)| {
        f64::from(*v) * *p
    }).sum();
    -(sum / f64::from(possible_words.len() as u32)).log2()
}

fn numtoword(word: u64) -> String {

    let word_bytes = vec![
        buf_i(word, 0),
        buf_i(word, 1),
        buf_i(word, 2),
        buf_i(word, 3),
        buf_i(word, 4),
    ];
    String::from_utf8(word_bytes).unwrap()
}

fn main() {
    let mut f = File::open("words.txt").unwrap();

    let mut word_file = String::new();
    f.read_to_string(&mut word_file).unwrap();
    drop(f);

    let mut words: Vec<u64> = Vec::new();

    let mut i = 0;
    while i < word_file.len() {
        let section = word_file[i..i + 6].as_bytes();
        let word: u64 = (section[0] as u64)
            + ((section[1] as u64) << 8)
            + ((section[2] as u64) << 16)
            + ((section[3] as u64) << 24)
            + ((section[4] as u64) << 32);
        words.push(word);
        i += 6;
    }

    let mut possible_words = words.clone();
    let found_out: &[(&str, [u8; 5])] = &[
        ("tares", [1,2,1,1,2]),
        ("erupt", [1,1,2,2,0]),
        ("remit", [1,1,2,2,0]),
    ];
    for f in found_out {
        possible_words = possible_words
            .par_iter()
            .filter(|w| matches_info(**w, guess_format(f.0.as_bytes()),
                                                       guess_format(&f.1)))
            .map(|x| *x)
            .collect();
    }
    println!("Words left: {}", possible_words.len());
    if possible_words.len() < 10 {
        println!("Possible words are: ");
        for word in possible_words.iter().map(|w| numtoword(*w)) {
            println!("\t{}", word)
        }
    }
    let mut infos: Vec<u64> = Vec::new();
    for i0 in 0..3 {
        for i1 in 0..3 {
            for i2 in 0..3 {
                for i3 in 0..3 {
                    for i4 in 0..3 {
                        let v = (i0 as u64)
                            + ((i1 as u64) << 8)
                            + ((i2 as u64) << 16)
                            + ((i3 as u64) << 24)
                            + ((i4 as u64) << 32);
                        infos.push(v)
                    }
                }
            }
        }
    }

    let sums: Vec<f64> = words.par_iter().map(|w| get_information(*w, &infos, &possible_words)).collect();
    let sums_boosted = sums.iter().enumerate().map(|(i, sum)| {
        sum + if possible_words.contains(&words[i]) {
            let frac = f64::from(possible_words.len() as i32) / f64::from(words.len() as i32);
            -frac.log2() / 10.0
        } else {
            0.0
        }
    });
    let (word, score) = words.iter().zip(sums_boosted).max_by(|(_, s), (_, s2)| {
        s.partial_cmp(s2).unwrap()
    }).unwrap();
    println!("{} ({})", numtoword(*word), score)
}