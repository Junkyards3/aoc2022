use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ComparisonResult{
    RightOrder,
    WrongOrder,
    Continue,
}

#[derive(Debug, Clone)]
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


pub fn day13() {
    let mut file = File::open("./inputs/input_day13t.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");


}