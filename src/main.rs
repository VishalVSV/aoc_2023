#![allow(dead_code)]

#[macro_use]
mod macros;
mod utils;
mod aoc_lib;

use std::{collections::{HashMap, VecDeque}, thread, sync::{atomic::AtomicUsize, Arc}};

use crate::aoc_lib::Grid;

use crate::utils::StringUtils;

fn main() {
    run!(thirteen);
}

day_c!(one, 
    |raw_data: String| {
        let mut sum: usize = 0;
        let mut sum2: usize = 0;
    
        for line in raw_data.lines() {
            let digit1 = (line.chars().find(|c| c.is_digit(10)).unwrap()) as usize - 48;
            let digit2 = (line.chars().rev().find(|c| c.is_digit(10)).unwrap()) as usize - 48;

            sum += digit1 * 10 + digit2;

            let digit1 = find_first_num(line);
            let digit2 = find_last_num(line);

            sum2 += digit1 * 10 + digit2;
        }

        (format!("{}", sum), format!("{}", sum2))
    }
);

fn find_first_num(num: &str) -> usize {
    let mut i = 0;

    let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for c in num.chars() {
        if c.is_digit(10) {
            return c as usize - 48;
        }
        else {
            for (j, dig) in nums.iter().enumerate() {
                if i + dig.len() < num.len() {
                    if &num[i..i+dig.len()] == *dig {
                        return j + 1;
                    }
                } 
            }
        }
        i += 1;
    }

    0
}

fn find_last_num(num: &str) -> usize {
    let mut i = num.len();

    let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for c in num.chars().rev() {
        if c.is_digit(10) {
            return c as usize - 48;
        }
        else {
            for (j, dig) in nums.iter().enumerate() {
                if i >= dig.len() {
                    if &num[i-dig.len()..i] == *dig {
                        return j + 1;
                    }
                } 
            }
        }
        i -= 1;
    }

    0
}



day!{two, 
    |raw_data: String| {
        let mut sum = 0;

        'game_loop: for line in raw_data.lines() {
            let (id, sets) = line.split_into(':').unwrap();

            for set in sets.split(';') {
                for cubes in set.split(',') {
                    let cubes = cubes.trim();
                    let (num, color) = cubes.split_into(' ').unwrap();
                    let num = num.parse::<usize>().unwrap();
                    if color == "blue" && num > 14 {
                        continue 'game_loop;
                    } else if color == "red" && num > 12 {
                        continue 'game_loop;
                    } else if color == "green" && num > 13 {
                        continue 'game_loop;
                    }
                }
            }

            let id = id[5..].parse::<usize>().unwrap();
            sum += id;
        }

        format!("{}", sum)
    },
    |raw_data: String| {let mut sum = 0;
        for line in raw_data.lines() {
            let (_, sets) = line.split_into(':').unwrap();

            let mut red_min = 0;
            let mut green_min = 0;
            let mut blue_min = 0;

            for set in sets.split(';') {
                for cubes in set.split(',') {
                    let cubes = cubes.trim();
                    let (num, color) = cubes.split_into(' ').unwrap();
                    let num = num.parse::<usize>().unwrap();
                    
                    match color {
                        "red" => {
                            red_min = std::cmp::max(red_min, num);
                        },
                        "green" => {
                            green_min = std::cmp::max(green_min, num);
                        }, 
                        "blue" => {
                            blue_min = std::cmp::max(blue_min, num);
                        },
                        _=>{}
                    }
                }
            }

            sum += red_min * blue_min * green_min;
        }

        format!("{}", sum)
    }
}



day!{three, 
    |raw_data: String| {
        let mut sum = 0;

        let mut lines = raw_data.lines().into_iter().peekable();
        let mut prev_line: Option<&str> = None;

        while let Some(line) = lines.next() {
            let mut num = None;
            let mut is_part_num = false;
            let mut last_c: Option<char> = None;
            let mut chars = line.chars().enumerate().peekable();

            while let Some((i, c)) = chars.next() {
                if c.is_digit(10) {
                    if num.is_none() {
                        num = Some(0);
                    }
                    if let Some(n) = num {
                        *num.as_mut().unwrap() = n * 10 + (c as usize - 48);
                        
                        if !is_part_num {
                            if let Some(prev_line) = prev_line {
                                let lower = if i > 0 { i - 1} else { 0 };
                                let upper = if i + 1 < prev_line.len() { i + 1 } else { prev_line.len() - 1 };
                                // println!("Checking: {} for {}", &prev_line[lower..=upper], c);
                                if prev_line[lower..=upper].find(|c: char| !c.is_digit(10) && c != '.').is_some() {
                                    is_part_num = true;
                                }
                            }
                            if let Some(last_c) = last_c {
                                if !last_c.is_digit(10) && last_c != '.' {
                                    is_part_num = true;
                                }
                            }
                            if let Some((_, next_c)) = chars.peek() {
                                if !next_c.is_digit(10) && *next_c != '.' {
                                    is_part_num = true;
                                }
                            }
                            if let Some(next_line) = &lines.peek() {
                                let lower = if i > 0 { i - 1} else { 0 };
                                let upper = if i + 1 < next_line.len() { i + 1 } else { next_line.len() - 1 };
                                // println!("Checking: {} for {}", &next_line[lower..=upper], c);
                                if next_line[lower..=upper].find(|c: char| !c.is_digit(10) && c != '.').is_some() {
                                    is_part_num = true;
                                }
                            }
                        }
                    }
                } else {
                    if let Some(num) = num {
                        if is_part_num {
                            sum += num;
                            print!("{}[31m{}{}[0m", 27 as char, num, 27 as char);
                        } else {
                            print!("{}", num);
                        }
                    }
                    if c == '.' {
                        print!("{}", c);
                    } else {
                        print!("{}[93m{}{}[0m", 27 as char, c, 27 as char);
                    }
                    num = None;
                    is_part_num = false;
                }
                last_c = Some(c);
            }

            if let Some(num) = num {
                if is_part_num {
                    sum += num;
                    print!("{}[31m{}{}[0m", 27 as char, num, 27 as char);
                } else {
                    print!("{}", num);
                }
            }

            println!();
            prev_line = Some(line);
        }

        format!("{}", sum)
    },
    |raw_data: String| {
        let mut sum = 0;

        let mut lines = raw_data.lines().into_iter().peekable();
        let mut prev_line: Option<&str> = None;

        let mut gears: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        let mut gear_list = vec![];

        let mut y = 0;
        while let Some(line) = lines.next() {
            let mut num = None;
            let mut is_part_num = false;
            let mut last_c: Option<char> = None;
            let mut chars = line.chars().enumerate().peekable();


            while let Some((i, c)) = chars.next() {
                if c.is_digit(10) {
                    if num.is_none() {
                        num = Some(0);
                    }
                    if let Some(n) = num {
                        *num.as_mut().unwrap() = n * 10 + (c as usize - 48);
                        
                        if !is_part_num {
                            if let Some(prev_line) = prev_line {
                                let lower = if i > 0 { i - 1} else { 0 };
                                let upper = if i + 1 < prev_line.len() { i + 1 } else { prev_line.len() - 1 };
                                // println!("Checking: {} for {}", &prev_line[lower..=upper], c);
                                if let Some(x) = prev_line[lower..=upper].find(|c: char| c == '*') {
                                    gear_list.push((x + lower, y - 1));
                                }
                            }
                            if let Some(last_c) = last_c {
                                if last_c == '*' {
                                    gear_list.push((i - 1, y));
                                }
                            }
                            if let Some((_, next_c)) = chars.peek() {
                                if *next_c == '*' {
                                    gear_list.push((i + 1, y));
                                }
                            }
                            if let Some(next_line) = &lines.peek() {
                                let lower = if i > 0 { i - 1} else { 0 };
                                let upper = if i + 1 < next_line.len() { i + 1 } else { next_line.len() - 1 };
                                // println!("Checking: {} for {}", &next_line[lower..=upper], c);
                                if let Some(x) = next_line[lower..=upper].find(|c: char| c == '*') {
                                    gear_list.push((x + lower, y + 1));
                                }
                            }
                        }
                    }
                } else {
                    if let Some(num) = num {
                        gear_list.dedup();
                        for gear in &gear_list {
                            let (count, gr) = *gears.entry(*gear).or_insert((0, 1));
                            *gears.entry(*gear).or_insert((0, 1)) = (count + 1, gr * num);
                        }
                    }
                    num = None;
                    gear_list.clear();
                    is_part_num = false;
                }
                last_c = Some(c);
            }
            prev_line = Some(line);
            y += 1;
        }

        for (_, (count, gr)) in gears {
            if count == 2 {
                sum += gr;
            }
        }

        format!("{}", sum)
    }
}



day!{four, 
    |raw_data: String| {
        let mut sum = 0;

        for line in raw_data.lines() {
            let mut score = None;

            let start = line.find(':').unwrap() + 1;
            let end = line.find('|').unwrap();
            let winning_nums = &line[start..end].trim();
            let winning_nums: Vec<usize> = winning_nums.split_whitespace().map(|a| a.parse::<usize>().unwrap()).collect();


            for num in line[end + 1..].split_whitespace().map(|a| a.parse::<usize>().unwrap()) {
                // print!("{} ", num);
                if winning_nums.contains(&num) {
                    match score {
                        Some(sc) => score = Some(sc * 2),
                        None => score = Some(1)
                    }
                }
            }
            sum += score.unwrap_or(0);
        }

        format!("{}", sum)
    },
    |raw_data: String| {

        let mut cards = vec![]; 

        let mut id = 1;
        for line in raw_data.lines() {
            let start = line.find(':').unwrap() + 1;
            let end = line.find('|').unwrap();
            let winning_nums = &line[start..end].trim();
            let winning_nums: Vec<usize> = winning_nums.split_whitespace().map(|a| a.parse::<usize>().unwrap()).collect();

            // let nums: Vec<usize> = line[end + 1..].trim().split_whitespace().map(|a| a.parse::<usize>().unwrap()).collect();
            let mut match_count = 0;

            for num in line[end + 1..].split_whitespace().map(|a| a.parse::<usize>().unwrap()) {
                // print!("{} ", num);
                if winning_nums.contains(&num) {
                    match_count += 1;
                }
            }

            cards.push(Card {
                id,
                count: 1,
                winning_numbers: winning_nums,
                match_count
            });

            id += 1;
        }

        for i in 0..cards.len() {
            for j in 1..=cards[i].match_count {
                if i + j < cards.len() {
                    cards[i + j].count += cards[i].count;
                }
            }
        }

        let sum: usize = cards.iter().map(|c| c.count).sum();

        format!("{}", sum)
    }
}

#[derive(Debug)]
struct Card {
    id: usize,
    count: usize,
    winning_numbers: Vec<usize>,
    match_count: usize
}

day_c!{five, 
    |raw_data: String| {
        let mut sum_1 = usize::MAX;

        let mut mappings: Vec<Vec<(usize, usize, usize)>> = vec![];

        let mut chunks = raw_data.split("\n\n");

        let seeds: Vec<usize> = chunks.next().unwrap()[7..].split_whitespace().map(|a| a.parse::<usize>().unwrap()).collect();

        for chunk in chunks {
            let mut mapping = vec![];
            for entry in chunk.lines().skip(1) {
                let mut tmp = entry.split_whitespace().map(|a| a.parse::<usize>().unwrap());
                let destination_range_start = tmp.next().unwrap();
                let source_range_start = tmp.next().unwrap();
                let range_size = tmp.next().unwrap();

                mapping.push((source_range_start, destination_range_start, range_size));
            }
            // println!("{:#?}", mapping);
            mappings.push(mapping);
        }

        for seed in &seeds {
            let mut num = *seed;
            'map_loop: for mapping in &mappings {
                for entry in mapping {
                    if entry.0 <= num && num < entry.0 + entry.2 {
                        num = entry.1 + (num - entry.0);
                        continue 'map_loop;
                    }
                }
            }
            sum_1 = std::cmp::min(sum_1, num);
        }

        let n = 100;

        let p2_min = AtomicUsize::new(usize::MAX);
        let p2_min = Arc::new(p2_min);
        
        for seed_range in seeds.chunks(2) {

            let d = seed_range[1] / n;
            let mut threads = vec![];

            for i in 0..n {
                let lower_bound = seed_range[0];
                let mappings = mappings.clone();
                let p2_min = p2_min.clone();
                threads.push(thread::spawn(move ||{
                    let mut sum_2 = usize::MAX;
                    for seed in lower_bound + i * d..lower_bound + (i + 1) * d {
                        let mut num = seed;
                        'map_loop: for mapping in &mappings {
                            for entry in mapping {
                                if entry.0 <= num && num < entry.0 + entry.2 {
                                    num = entry.1 + (num - entry.0);
                                    continue 'map_loop;
                                }
                            }
                        }
        
                        sum_2 = std::cmp::min(sum_2, num);
                        // if seed == seed_range[0] || (seed - seed_range[0]) % 1000000 == 0 {
                        //     println!("{}", num);
                        // }
                        // print!("\r{}", (seed - seed_range[0]) as f64 / (seed_range[1]) as f64);
                    }

                    _ = p2_min.fetch_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |p| {
                        Some(if p > sum_2 {
                            sum_2
                        } else {
                            p
                        })
                    });
                }));
            }

            for th in threads {
                th.join().unwrap();
            }
        }
        
        
        (format!("{}", sum_1), format!("{:?}", p2_min))
    }
}



day!(six, 
    |raw_data: String| {
        let mut sum = 1;
        let races = {
            let mut lines = raw_data.lines();
            let line1 = lines.next().unwrap();
            let line2 = lines.next().unwrap();

            line1.split_whitespace().skip(1).map(|a| a.parse::<usize>().unwrap()).zip(line2.split_whitespace().skip(1).map(|a| a.parse::<usize>().unwrap()))
        };

        for (time, distance) in races {
            let mut candidates = 0;
            for i in 0..time {
                let distance_travelled = (time - i) * (i);
                if distance_travelled > distance {
                    candidates += 1;
                }
            }
            sum *= candidates;
        }

        format!("{}", sum)
    },
    |raw_data: String| {
        let (time, distance) = {
            let mut lines = raw_data.lines();
            let line1 = lines.next().unwrap();
            let line2 = lines.next().unwrap();

            (line1.split_whitespace().skip(1).fold(String::new(), |mut a, b| {
                a.reserve(b.len() + 1);
                a.push_str(b);
                a
            }).parse::<usize>().unwrap(),
            line2.split_whitespace().skip(1).fold(String::new(), |mut a, b| {
                a.reserve(b.len() + 1);
                a.push_str(b);
                a
            }).parse::<usize>().unwrap())
        };

        println!("{} {}", time, distance);

        let mut candidates = 0;
        for i in 0..time {
            let distance_travelled = (time - i) * (i);
            if distance_travelled > distance {
                candidates = i;
                break;
            }
        }


        for i in (0..time).rev() {
            let distance_travelled = (time - i) * (i);
            if distance_travelled > distance {
                candidates = i - candidates;
                break;
            }
        }

        // format!("{}", sum)

        format!("{}", candidates + 1)
    }
);

day!(seven,
    |raw_data: String| {
        // 254147397 Too High
        // 253954294

        let mut list_of_cards = vec![];

        for line in raw_data.lines() {
            let (card, bid) = line.split_into(' ').unwrap();
            list_of_cards.push((D7Hand::parse_p1(card).unwrap(), bid.parse::<usize>().unwrap()));
        }

        list_of_cards.sort_by(|a, b| a.0.my_cmp_p1(&b.0));

        // println!("{:?}", list_of_cards);

        let res = list_of_cards.iter().enumerate().fold(0, |acc, elem| {
            // println!("{} += {} * {}", acc, elem.0 + 1, elem.1.1);
            
            acc + (elem.0 + 1) * elem.1.1
        });

        format!("{}", res)
    },
    |raw_data: String| {
        let mut list_of_cards = vec![];

        for line in raw_data.lines() {
            let (card, bid) = line.split_into(' ').unwrap();
            list_of_cards.push((D7Hand::parse_p2(card), bid.parse::<usize>().unwrap()));
        }

        list_of_cards.sort_by(|a, b| a.0.my_cmp_p2(&b.0));

        // println!("{:?}", list_of_cards);

        let res = list_of_cards.iter().enumerate().fold(0, |acc, elem| {            
            acc + (elem.0 + 1) * elem.1.1
        });

        format!("{}", res)
    }
);

impl D7Hand {
    fn parse_p1(s: &str) -> Result<Self, ()> {
        let mut chars: HashMap<char, usize> = HashMap::new();

        for c in s.chars() {
            *chars.entry(c).or_insert(0) += 1
        }

        let s = s.to_owned();
        if chars.len() == 5 {
            Ok(Self::HighCard(s))
        } else if chars.len() == 4 {
            Ok(Self::OnePair(s))
        } else if chars.len() == 3 {
            if chars.values().any(|&x| x  == 3) {
                Ok(Self::ThreeOfAKind(s))
            }
            else {
                Ok(Self::TwoPair(s))
            }
        } else if chars.len() == 2 {
            if chars.values().any(|&x| x  == 4) {
                Ok(Self::FourOfAKind(s))
            }
            else {
                Ok(Self::FullHouse(s))
            }
        } else if chars.len() == 1 {
            Ok(Self::FiveOfAKind(s))
        }
        else {
            Err(())
        }
    }

    fn parse_p2(s: &str) -> Self {
        let mut chars_raw: HashMap<char, usize> = HashMap::new();

        for c in s.chars() {
            *chars_raw.entry(c).or_insert(0) += 1
        }

        let mut possible_hands = vec![];

        let chars = chars_raw.clone();
        let s1 = s.to_owned();

        possible_hands.push(
        if chars.len() == 5 {
            Self::HighCard(s1)
        } else if chars.len() == 4 {
            Self::OnePair(s1)
        } else if chars.len() == 3 {
            if chars.values().any(|&x| x  == 3) {
                Self::ThreeOfAKind(s1)
            }
            else {
                Self::TwoPair(s1)
            }
        } else if chars.len() == 2 {
            if chars.values().any(|&x| x  == 4) {
                Self::FourOfAKind(s1)
            }
            else {
                Self::FullHouse(s1)
            }
        } else if chars.len() == 1 {
            Self::FiveOfAKind(s1)
        }
        else {
            unreachable!()
        });

        for jkr in s.chars() {
            if jkr == 'J' {
                continue;
            }
            let mut chars = chars_raw.clone();
            *chars.entry(jkr).or_insert(0) += chars_raw.get(&'J').unwrap_or(&0);
            chars.remove(&'J');

            let s = s.to_owned();

            possible_hands.push(
            if chars.len() == 5 {
                Self::HighCard(s)
            } else if chars.len() == 4 {
                Self::OnePair(s)
            } else if chars.len() == 3 {
                if chars.values().any(|&x| x  == 3) {
                    Self::ThreeOfAKind(s)
                }
                else {
                    Self::TwoPair(s)
                }
            } else if chars.len() == 2 {
                if chars.values().any(|&x| x  == 4) {
                    Self::FourOfAKind(s)
                }
                else {
                    Self::FullHouse(s)
                }
            } else if chars.len() == 1 {
                Self::FiveOfAKind(s)
            }
            else {
                unreachable!()
            });
        }

        possible_hands.iter().max_by(|a, b| a.implicit_worth().cmp(&b.implicit_worth())).unwrap().clone()
    }
}

impl D7Hand {
    fn implicit_worth(&self) -> usize {
        match self {
            D7Hand::FiveOfAKind(_) => 7,
            D7Hand::FourOfAKind(_) => 6,
            D7Hand::FullHouse(_) => 5,
            D7Hand::ThreeOfAKind(_) => 4,
            D7Hand::TwoPair(_) => 3,
            D7Hand::OnePair(_) => 2,
            D7Hand::HighCard(_) => 1,
        }
    }

    fn raw(&self) -> &String {
        match self {
            D7Hand::FiveOfAKind(w) => w,
            D7Hand::FourOfAKind(w) => w,
            D7Hand::FullHouse(w) => w,
            D7Hand::ThreeOfAKind(w) => w,
            D7Hand::TwoPair(w) => w,
            D7Hand::OnePair(w) => w,
            D7Hand::HighCard(w) => w,
        }
    }
}

fn strength_of_card_p1(c: char) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 1
    }
}

fn strength_of_card_p2(c: char) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => 0
    }
}

impl D7Hand {
    fn my_cmp_p1(&self, other: &Self) -> std::cmp::Ordering {
        match self.implicit_worth().cmp(&other.implicit_worth()) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                let mut res = std::cmp::Ordering::Equal;
                for (a, b) in self.raw().chars().zip(other.raw().chars()) {
                    if a != b {
                        let (a, b) = (strength_of_card_p1(a), strength_of_card_p1(b));
                        res = a.cmp(&b);
                        break;
                    }
                }
                res
            },
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater
        }
    }

    fn my_cmp_p2(&self, other: &Self) -> std::cmp::Ordering {
        match self.implicit_worth().cmp(&other.implicit_worth()) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                let mut res = std::cmp::Ordering::Equal;
                for (a, b) in self.raw().chars().zip(other.raw().chars()) {
                    if a != b {
                        let (a, b) = (strength_of_card_p2(a), strength_of_card_p2(b));
                        res = a.cmp(&b);
                        break;
                    }
                }
                res
            },
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum D7Hand {
    FiveOfAKind(String),
    FourOfAKind(String),
    FullHouse(String),
    ThreeOfAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String)
}



day!(eight,
    |raw_data: String| {
        // return format!("");

        let mut lines = raw_data.lines();

        let instructions: Vec<char> = lines.next().unwrap().chars().collect();

        lines.next();

        let mut map: HashMap<String, (String, String)> = HashMap::new();

        for line in lines {
            map.insert(line[0..3].to_owned(), (line[7..10].to_owned(), line[12..15].to_owned()));
        }

        let mut state = "AAA".to_owned();
        let mut steps = 0;
        loop {
            let branch = map.get(&state).unwrap();

            match instructions[steps % instructions.len()] {
                'L' => {
                    state = branch.0.clone();
                },
                'R' => {
                    state = branch.1.clone();
                }, _=> unreachable!()
            }

            steps += 1;
            if state == "ZZZ" {
                break;
            }
        }

        format!("{}", steps)
    },
    |raw_data: String| {
        let mut lines = raw_data.lines();

        let instructions: Vec<char> = lines.next().unwrap().chars().collect();

        lines.next();

        let mut map: HashMap<String, (String, String)> = HashMap::new();

        for line in lines {
            map.insert(line[0..3].to_owned(), (line[7..10].to_owned(), line[12..15].to_owned()));
        }

        let mut res = vec![];

        for start in map.keys().filter(|a| a.ends_with("A")) {
            let mut state = start.clone();
            let mut steps = 0;
            loop {
                let branch = map.get(&state).unwrap();

                match instructions[steps % instructions.len()] {
                    'L' => {
                        state = branch.0.clone();
                    },
                    'R' => {
                        state = branch.1.clone();
                    }, _=> unreachable!()
                }

                steps += 1;
                if state.ends_with("Z") {
                    break;
                }
            }
            res.push(steps);
        }

        fn gcd(a: usize, b: usize) -> usize {
            let mut x = a;
            let mut y = b;
            while x != y {
                if x > y {
                    x -= y;
                } else {
                    y -= x
                }
            }
            x
        }

        let mut result = 1;
        for i in res {
            result = (i * result) / gcd(i, result);
        }
        // Scum result took hint from internet shame on me
        format!("{}", result)
    }
);


day_c!(nine, 
    |raw_data: String| {
        let mut res_p1 = 0;
        let mut res_p2 = 0;

        for line in raw_data.lines() {
            let mut local_res_p2 = 0;

            let mut matrix = vec![];
            matrix.push(line.split_whitespace().map(|a| a.parse::<isize>().unwrap()).collect::<Vec<isize>>());
            let len = matrix[0].len();
            matrix.resize(len, vec![0; len]);
            
            let mut last_row = 0;
            for row in 1..matrix[0].len() {
                let mut all_zeros = true;

                for i in 0..len - row {
                    matrix[row][i] = matrix[row - 1][i + 1] - matrix[row - 1][i];

                    if matrix[row][i] != 0 {
                        all_zeros = false;
                    }
                }

                if all_zeros {
                    last_row = row;
                    break;
                }
            }
            
            for i in (0..=last_row).rev() {
                res_p1 += matrix[i][len - 1 - i];
                // println!("{} - {} = {}", matrix[i][0], local_res_p2, matrix[i][0] - local_res_p2);
                local_res_p2 = matrix[i][0] - local_res_p2;
            }

            // println!("{}", local_res_p2);
            res_p2 += local_res_p2;
        }

        (format!("{}", res_p1), format!("{}", res_p2))
    }
);

day_c!(ten,
    |raw_data: String| {
        let mut lines = raw_data.lines();

        let line = lines.next().unwrap();

        let mut grid: Grid<D10CellState> = Grid::new(line.len(), line.len());

        let mut row = 0;
        let mut col = 0;

        for c in line.chars() {
            match c {
                '|' => {
                    grid[row][col] = D10CellState::VerticalPipe;
                    col += 1;
                },
                '-' => {
                    grid[row][col] = D10CellState::HorizontalPipe;
                    col += 1;
                },
                'L' => {
                    grid[row][col] = D10CellState::NEBend;
                    col += 1;
                },
                'J' => {
                    grid[row][col] = D10CellState::NWBend;
                    col += 1;
                },
                '7' => {
                    grid[row][col] = D10CellState::SWBend;
                    col += 1;
                },
                'F' => {
                    grid[row][col] = D10CellState::SEBend;
                    col += 1;
                },
                '.' => {
                    grid[row][col] = D10CellState::Ground;
                    col += 1;
                },
                'S' => {
                    grid[row][col] = D10CellState::StartingPos;
                    col += 1;
                },
                _ => unreachable!()
            }
        }
        col = 0;
        row += 1;

        let mut starting_pos_row = 0;
        let mut starting_pos_col = 0;

        for line in lines {
            for c in line.chars() {
                match c {
                    '|' => {
                        grid[row][col] = D10CellState::VerticalPipe;
                        col += 1;
                    },
                    '-' => {
                        grid[row][col] = D10CellState::HorizontalPipe;
                        col += 1;
                    },
                    'L' => {
                        grid[row][col] = D10CellState::NEBend;
                        col += 1;
                    },
                    'J' => {
                        grid[row][col] = D10CellState::NWBend;
                        col += 1;
                    },
                    '7' => {
                        grid[row][col] = D10CellState::SWBend;
                        col += 1;
                    },
                    'F' => {
                        grid[row][col] = D10CellState::SEBend;
                        col += 1;
                    },
                    '.' => {
                        grid[row][col] = D10CellState::Ground;
                        col += 1;
                    },
                    'S' => {
                        grid[row][col] = D10CellState::StartingPos;
                        starting_pos_col = col;
                        starting_pos_row = row;
                        col += 1;
                    },
                    _ => unreachable!()
                }
            }
            col = 0;
            row += 1;
        }

        grid.data.resize(row, vec![]);
        grid.height = row;

        row = starting_pos_row;
        col = starting_pos_col;

        // Identifying the loop
        macro_rules! adjacent_connections {
            ($current_pos: expr) => {
                {
                    let (row, col) = $current_pos;
                    let mut connections = vec![];
                    match grid[row][col] {
                        D10CellState::VerticalPipe => {
                            if row > 0 {
                                connections.push((row - 1,col));
                            }
                            if row + 1 < grid.height {
                                connections.push((row + 1,col));
                            }
                        },
                        D10CellState::HorizontalPipe => {
                            if col > 0 {
                                connections.push((row,col - 1));
                            }
                            if col + 1 < grid.width {
                                connections.push((row,col + 1));
                            }
                        },
                        D10CellState::NEBend => {
                            if row > 0 {
                                connections.push((row - 1,col));
                            }
                            if col + 1 < grid.width {
                                connections.push((row,col + 1));
                            }
                        },
                        D10CellState::NWBend => {
                            if row > 0 {
                                connections.push((row - 1,col));
                            }
                            if col > 0 {
                                connections.push((row,col - 1));
                            }
                        },
                        D10CellState::SWBend => {
                            if row + 1 < grid.height {
                                connections.push((row + 1,col));
                            }
                            if col > 0 {
                                connections.push((row,col - 1));
                            }
                        },
                        D10CellState::SEBend => {
                            if row + 1 < grid.height {
                                connections.push((row + 1,col));
                            }
                            if col + 1 < grid.width {
                                connections.push((row,col + 1));
                            }
                        },
                        D10CellState::Ground => {},
                        D10CellState::StartingPos => {
                            if row > 0 {
                                match grid[row - 1][col] {
                                    D10CellState::VerticalPipe | D10CellState::SEBend | D10CellState::SWBend => connections.push((row - 1,col)),
                                    _ => {}
                                }
                            }
                            if row + 1 < grid.height {
                                match grid[row + 1][col] {
                                    D10CellState::VerticalPipe | D10CellState::NEBend | D10CellState::NWBend => connections.push((row + 1,col)),
                                    _ => {}
                                }
                            }
                            if col > 0 {
                                match grid[row][col - 1] {
                                    D10CellState::HorizontalPipe | D10CellState::SEBend | D10CellState::NEBend => connections.push((row,col - 1)),
                                    _ => {}
                                }
                            }
                            if col + 1 < grid.width {
                                match grid[row][col + 1] {
                                    D10CellState::HorizontalPipe | D10CellState::SWBend | D10CellState::NWBend => connections.push((row,col + 1)),
                                    _ => {}
                                }
                            }
                        }
                    }
                    connections
                }
            }
        }
        

        let mut visited_positions: Vec<((usize, usize), usize)> = vec![];
        let mut visiting_positions: VecDeque<((usize, usize), usize)> = VecDeque::new();

        visiting_positions.push_back(((row, col), 0));

        'outer: while visiting_positions.len() > 0 {
            let (current_pos, current_distance) = visiting_positions.pop_front().unwrap();

            for (p, old_dist) in &visited_positions {
                if p == &current_pos {
                    if *old_dist > current_distance {
                        // todo
                    }
                    continue 'outer;
                }
            }

            visited_positions.push((current_pos, current_distance));

            for adjacent in adjacent_connections!(current_pos) {
                visiting_positions.push_back((adjacent, current_distance + 1));
            }
        }

        let ad = adjacent_connections!((starting_pos_row, starting_pos_col));
        if starting_pos_col + 1 < grid.width && ad.contains(&(starting_pos_row, starting_pos_col + 1)) {
            if starting_pos_row > 0 && ad.contains(&(starting_pos_row - 1, starting_pos_col)) {
                grid[starting_pos_row][starting_pos_col] = D10CellState::NEBend;
            } else if starting_pos_row + 1 < grid.height && ad.contains(&(starting_pos_row + 1, starting_pos_col)) {
                grid[starting_pos_row][starting_pos_col] = D10CellState::SEBend;
            } else {
                grid[starting_pos_row][starting_pos_col] = D10CellState::HorizontalPipe;
            }
        } else {
            if starting_pos_col > 0 && ad.contains(&(starting_pos_row, starting_pos_col - 1)) {
                if starting_pos_row > 0 && ad.contains(&(starting_pos_row - 1, starting_pos_col)) {
                    grid[starting_pos_row][starting_pos_col] = D10CellState::NWBend;
                } else {
                    grid[starting_pos_row][starting_pos_col] = D10CellState::SWBend;
                }
            } else {
                grid[starting_pos_row][starting_pos_col] = D10CellState::VerticalPipe;
            }
        }


        // println!("{:?}", visited_positions);

        let mut res2 = 0;

        let mut inside = vec![];
        for i in 0..grid.height {
            let mut in_region = false;
            let mut scratch_cell = D10CellState::VerticalPipe;
            for j in 0..grid.width {
                if let Some((_, _)) = visited_positions.iter().find(|a| a.0 == (i, j)) {
                    match grid[i][j] {
                        D10CellState::VerticalPipe => in_region = !in_region,
                        D10CellState::NEBend => {
                            scratch_cell = D10CellState::NEBend;
                        },
                        D10CellState::SEBend => {
                            scratch_cell = D10CellState::SEBend;
                        },
                        D10CellState::NWBend => {
                            if let D10CellState::SEBend = scratch_cell {
                                in_region = !in_region;
                            }
                        },
                        D10CellState::SWBend => {
                            if let D10CellState::NEBend = scratch_cell {
                                in_region = !in_region;
                            }
                        },
                        _ => {}
                    }
                }
                else {
                    if in_region {
                        res2 += 1;
                        inside.push((i, j));
                    }
                }

                
            }
        }

        // println!("{}", res2);

        print_grid(&grid, &visited_positions, &inside);
        

        (format!("{}", visited_positions.iter().map(|a| a.1).max().unwrap()), format!("{}", res2))
    }
);

fn print_grid(grid: &Grid<D10CellState>, visited_positions: &Vec<((usize, usize), usize)>, inside: &Vec<(usize, usize)>) {
    
    for i in 0..grid.height {
        for j in 0..grid.width {
            let cell = match grid[i][j] {
                D10CellState::VerticalPipe => format!("│"),
                D10CellState::HorizontalPipe => format!("─"),
                D10CellState::NEBend => format!("└"),
                D10CellState::NWBend => format!("┘"),
                D10CellState::SWBend => format!("┐"),
                D10CellState::SEBend => format!("┌"),
                D10CellState::Ground => format!("."),
                D10CellState::StartingPos => format!("S"),
            };
            if inside.contains(&(i, j)) {
                print!("{0}[37mI{0}[0m", 27 as char);
            } else {
                if let Some((_, _distance)) = visited_positions.iter().find(|a| a.0 == (i, j)) {
                    print!("{0}[31m{1}{0}[0m", 27 as char, cell);
                } else {
                    print!("{}", cell);
                }
            }
        }
        println!();
    }
}

#[derive(Clone, Copy, Default)]
enum D10CellState {
    #[default] VerticalPipe,
    HorizontalPipe,
    NEBend,
    NWBend,
    SWBend,
    SEBend,
    Ground,
    StartingPos
}



day!(eleven, 
    |raw_data: String| {
        let mut grid: Vec<Vec<bool>> = vec![];

        for line in raw_data.lines() {
            let mut grid_line = vec![];

            for c in line.chars() {
                grid_line.push(c == '#');
            }

            grid.push(grid_line)
        }

        let mut width = grid[0].len();
        let mut height = grid.len();

        let mut expansion = vec![];

        for i in 0..width {
            let mut empty = true;
            for j in 0..height {
                empty = empty && !grid[j][i];
            }
            if empty {
                expansion.push(i);
            }
        }

        println!("{:?}", expansion);

        expansion.reverse();

        // Columns
        for idx in &expansion {
            for i in 0..height {
                grid[i].insert(*idx, false);
            }
            width += 1;
        }

        expansion.clear();

        for i in 0..height {
            let mut empty = true;
            for j in 0..width {
                empty = empty && !grid[i][j];
            }
            if empty {
                expansion.push(i);
            }
        }

        expansion.reverse();

        // Rows
        for idx in &expansion {
            grid.insert(*idx, vec![false; width]);
            height += 1;
        }


        let mut x: usize = 0;
        let mut y: usize = 0;
        
        let mut galaxies = vec![];
        for line in &grid {
            for c in line {
                print!("{}", if *c { galaxies.push((x, y)); '#' } else { '.' });
                x += 1;
            }
            println!();
            y += 1;
            x = 0;
        }

        let mut res = 0;
        for i in 0..galaxies.len() - 1 {
            for j in i + 1..galaxies.len() {
                let dist = (galaxies[i].0.abs_diff(galaxies[j].0)) + (galaxies[i].1.abs_diff(galaxies[j].1));
                // println!("{}", dist);
                
                // x = 0;
                // y = 0;
                // for line in &grid {
                //     for _ in line {
                //         print!("{}", if (x == galaxies[i].0 && y == galaxies[i].1) || (x == galaxies[j].0 && y == galaxies[j].1) { '#' } else { '.' });
                //         x += 1;
                //     }
                //     println!();
                //     y += 1;
                //     x = 0;
                // }

                // println!();

                res += dist;
            }
        }

        format!("{}", res)
    },
    |raw_data: String| {
        let mut grid: Vec<Vec<bool>> = vec![];

        for line in raw_data.lines() {
            let mut grid_line = vec![];

            for c in line.chars() {
                grid_line.push(c == '#');
            }

            grid.push(grid_line)
        }

        let width = grid[0].len();
        let height = grid.len();

        let mut expansion_cols = vec![];

        for i in 0..width {
            let mut empty = true;
            for j in 0..height {
                empty = empty && !grid[j][i];
            }
            if empty {
                expansion_cols.push(i);
            }
        }

        println!("{:?}", expansion_cols);

        let mut expansion_rows = vec![];

        for i in 0..height {
            let mut empty = true;
            for j in 0..width {
                empty = empty && !grid[i][j];
            }
            if empty {
                expansion_rows.push(i);
            }
        }


        let mut x: usize = 0;
        let mut y: usize = 0;
        
        let mut galaxies = vec![];
        for line in &grid {
            for c in line {
                print!("{}", if *c { galaxies.push((x, y)); '#' } else { '.' });
                x += 1;
            }
            println!();
            y += 1;
            x = 0;
        }

        // println!("{:?}", galaxies);

        let mut res = 0;
        for i in 0..galaxies.len() - 1 {
            for j in i + 1..galaxies.len() {
                let dist = dist_btw(galaxies[i], galaxies[j], &expansion_cols, &expansion_rows, 1000000);

                res += dist;
            }
        }

        format!("{}", res)
    }
);

fn dist_btw(p1: (usize, usize), p2: (usize, usize), expansion_cols: &Vec<usize>, expansion_rows: &Vec<usize>, expansion_factor: usize) -> usize {
    let mut start = std::cmp::min(p1.0, p2.0);
    let mut end = std::cmp::max(p1.0, p2.0);

    let mut distance = 0;

    for i in start..end {
        if expansion_cols.contains(&i) {
            distance += expansion_factor;
        } else {
            distance += 1;
        }
    }

    start = std::cmp::min(p1.1, p2.1);
    end = std::cmp::max(p1.1, p2.1);


    for i in start..end {
        if expansion_rows.contains(&i) {
            distance += expansion_factor;
        } else {
            distance += 1;
        }
    }

    distance
}

day!(twelve,
   |raw_data: String| {
        let mut res = 0;

        for line in raw_data.lines() {
            let con_record = D12ConditionRecord::from_string(line.to_owned());

            res += con_record.valid_possibilities();
        }

       format!("{}", res)
   },
   |raw_data: String| {
        let mut res = 0;

        for line in raw_data.lines() {
            let con_record = D12ConditionRecord::from_string_p2(line.to_owned());

            println!("{:?}", con_record);
            // res += con_record.valid_possibilities();
        }
       format!("")
});

#[derive(Debug)]
struct D12ConditionRecord {
    pub condition_map: Vec<char>,
    pub condition_map_alt: Vec<usize>
}

impl D12ConditionRecord {
    pub fn from_string(s: String) -> Self {
        let (con_map, con_map_alt) = s.split_into(' ').unwrap();

        let condition_map = con_map.chars().collect();
        let condition_map_alt = con_map_alt.split(',').map(|a| a.parse().unwrap()).collect();


        Self {
            condition_map,
            condition_map_alt
        }
    }


    pub fn from_string_p2(s: String) -> Self {
        let (con_map, con_map_alt) = s.split_into(' ').unwrap();

        let condition_map_tmp: Vec<char> = con_map.chars().collect();
        let condition_map_alt_tmp: Vec<usize> = con_map_alt.split(',').map(|a| a.parse().unwrap()).collect();

        let mut condition_map = vec![];
        let mut condition_map_alt = vec![];

        for _ in 0..5 {
            condition_map.extend(condition_map_tmp.iter());
            condition_map.push('?');

            condition_map_alt.extend(condition_map_alt_tmp.iter());
        }

        Self {
            condition_map,
            condition_map_alt
        }
    }

    pub fn is_valid(condition_map: &Vec<char>, condition_map_alt: &Vec<usize>) -> bool {
        let mut con_map_alt: Vec<usize> = vec![];

        let mut counter = 0;
        for c in condition_map {
            if *c == '#' {
                counter += 1;
            } else {
                if counter != 0 {
                    con_map_alt.push(counter);
                }
                counter = 0;
            }
        }

        if counter != 0 {
            con_map_alt.push(counter);
        }

        // println!("{:?}", con_map_alt);

        &con_map_alt == condition_map_alt
    }

    pub fn valid_possibilities(&self) -> usize {
        let mut possibilities = vec![];
        possibilities.push(self.condition_map.clone());

        let mut done = 0;

        'p_loop: while let Some(con_map) = possibilities.pop() {
            for (i, c) in con_map.iter().enumerate() {
                if *c == '?' {
                    let mut clone_a = con_map.clone();
                    let mut clone_b = con_map.clone();

                    clone_a[i] = '#';
                    clone_b[i] = '.';

                    possibilities.push(clone_a);
                    possibilities.push(clone_b);

                    continue 'p_loop;
                }
            }
            if Self::is_valid(&con_map, &self.condition_map_alt) {
                done += 1;
            }
        }


        done
    }
}


day!(thirteen,
    |raw_data: String| {
        let mut patterns = vec![];
        for pattern in raw_data.split("\n\n") {
            let mut pattern_c = vec![];

            for line in pattern.lines() {
                let row: Vec<char> = line.chars().collect();
                pattern_c.push(row);
            }

            patterns.push(pattern_c);
        }

        for pattern in patterns {
            // Vertical Line
            // let mut pivot = None;
            for i in 0..pattern.len() {
                
            }
        }

        format!("")
    },
    |raw_data: String| {

        format!("")
});