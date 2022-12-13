use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ComparisonResult{
    RightOrder,
    WrongOrder,
    Continue,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum ListElement {
    Int(u32),
    List(Vec<ListElement>)
}

impl ListElement {
    fn compare(&self, other: &ListElement) -> ComparisonResult {
        match (self,other) {
            (ListElement::Int(v1), ListElement::Int(v2)) => {
                match v1.cmp(v2) {
                    Ordering::Less => ComparisonResult::RightOrder,
                    Ordering::Equal => ComparisonResult::Continue,
                    Ordering::Greater => ComparisonResult::WrongOrder
                }
            }
            (ListElement::Int(v1), ListElement::List(l2)) => {
                ListElement::List(vec![ListElement::Int(*v1)]).compare(&ListElement::List(l2.clone()))
            }
            (ListElement::List(l1),ListElement::Int(v2)) => {
                ListElement::List(l1.clone()).compare(&ListElement::List(vec![ListElement::Int(*v2)]))
            }
            (ListElement::List(l1),ListElement::List(l2)) => {
                let mut it1 = l1.iter();
                let mut it2 = l2.iter();
                loop {
                    let option_le1 = it1.next();
                    let option_le2 = it2.next();
                    match (option_le1, option_le2) {
                        (Some(le1), Some(le2)) => {
                            let result = le1.compare(le2);
                            if result != ComparisonResult::Continue {
                                return result;
                            }
                        },
                        (Some(_),None) => return ComparisonResult::WrongOrder,
                        (None,Some(_)) => return ComparisonResult::RightOrder,
                        (None,None) => return ComparisonResult::Continue,
                    }
                }
            }
        }
    }
}

impl PartialOrd<Self> for ListElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.compare(other) {
            ComparisonResult::RightOrder => Ordering::Less,
            ComparisonResult::WrongOrder => Ordering::Greater,
            ComparisonResult::Continue => Ordering::Equal,
        })
    }
}

impl Ord for ListElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
fn split_by_correct_commas(s: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut nb_opened_brackets = 0;
    let mut buffer = String::new();
    s.chars()
        .for_each(|c| {
            match c {
                '[' => {
                    nb_opened_brackets += 1;
                    buffer.push(c) ;
                }
                ']' => {
                    nb_opened_brackets -= 1;
                    buffer.push(c) ;
                }
                ',' => {
                    if nb_opened_brackets == 0 {
                        result.push(buffer.clone());
                        buffer = String::new();
                    }
                    else{
                        buffer.push(c);
                    }
                }
                c => {
                    buffer.push(c) ;
                }
            }
        });
    if !buffer.is_empty() {
        result.push(buffer.clone());
    }

    result
}
fn parse(s: &str) -> ListElement {
    if s.starts_with('[') {
        //last char must be ]
        //first : find the correct commas
        let v: Vec<ListElement> = split_by_correct_commas(&s[1..s.len()-1])
            .iter()
            .map(|s| parse(s))
            .collect();
        ListElement::List(v)
    } else {
        let nb = s.parse::<u32>().unwrap();
        ListElement::Int(nb)
    }
}

pub fn day13() {
    let mut file = File::open("./inputs/input_day13.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let sol1 = data.split("\n\n")
        .enumerate()
        .filter_map(|(i,pair)| {
            let mut iter_pair = pair.split('\n');
            let le1 = parse(iter_pair.next().unwrap());
            let le2 = parse(iter_pair.next().unwrap());
            match le1.compare(&le2) {
                ComparisonResult::RightOrder => Some(i+1),
                _ => None,
            }
        })
        .sum::<usize>();

    let mut v = data
        .split_whitespace()
        .map(parse)
        .collect::<Vec<ListElement>>();

    let le2 = parse("[[2]]");
    let le6 = parse("[[6]]");

    v.push(le2.clone());
    v.push(le6.clone());
    v.sort_unstable();

    let p2 = v.iter().position(|le| *le == le2).unwrap() + 1;
    let p6 = v.iter().position(|le| *le == le6).unwrap() + 1;
    println!("Solution 1 : {}",sol1);
    println!("Solution 2 : {:?}",p2*p6);
}