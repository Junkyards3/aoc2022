use std::fs::File;
use std::io::Read;
use itertools::Itertools;
use ndarray::{Array2, Axis, Dimension};

#[derive(Debug,Clone)]
struct Tree {
    height: u32,
    visible_from_left: bool,
    visible_from_right: bool,
    visible_from_top: bool,
    visible_from_bottom: bool
}

impl Default for Tree {
    fn default() -> Self {
        Tree::make_tree(0)
    }
}
impl Tree {
    fn is_visible_from_outside(&self) -> bool {
        self.visible_from_bottom || self.visible_from_left || self.visible_from_top || self.visible_from_right
    }

    fn make_tree(height : u32) -> Self {
        Tree {
            height,
            visible_from_left: true,
            visible_from_right: true,
            visible_from_top: true,
            visible_from_bottom: true,
        }
    }
}
#[derive(Debug)]
struct Trees {
    trees: Array2<Tree>
}

impl Trees {
    fn set_hiding(&mut self, i0: usize, j0: usize) {
        let height_hiding_tree = self.trees[[i0,j0]].height;
        let (height,width) = self.trees.raw_dim().into_pattern();
        //above (vis from bot)
        for k in 0..i0 {
            if self.trees[[k,j0]].height <= height_hiding_tree {
                self.trees[[k, j0]].visible_from_bottom = false;
            }
        }
        //below (vis from top)
        for k in i0+1..height {
            if self.trees[[k,j0]].height <= height_hiding_tree {
                self.trees[[k, j0]].visible_from_top = false;
            }
        }
        //left (vis from right)
        for k in 0..j0 {
            if self.trees[[i0,k]].height <= height_hiding_tree {
                self.trees[[i0,k]].visible_from_right = false;
            }
        }
        //right (vis from left)
        for k in j0+1..width {
            if self.trees[[i0,k]].height <= height_hiding_tree {
                self.trees[[i0,k]].visible_from_left = false;
            }
        }
    }

    fn compute_scenic_score(&self,i0: usize, j0: usize) -> usize {
        let height_hiding_tree = self.trees[[i0,j0]].height;
        let (height,width) = self.trees.raw_dim().into_pattern();
        let mut result = 1;
        //above
        let mut c = (0..i0).rev()
            .take_while(|&i| self.trees[[i,j0]].height < height_hiding_tree)
            .count();
        if c == i0 {
            result *= c
        }
        else {
            result *= c + 1
        }
        //below
        c = (i0+1..height)
            .take_while(|&i| self.trees[[i,j0]].height < height_hiding_tree)
            .count();
        if c == (height - (i0 + 1)) {
            result *= c
        }
        else {
            result *= c + 1
        }
        //left
        c = (0..j0).rev()
            .take_while(|&j| self.trees[[i0,j]].height < height_hiding_tree)
            .count();
        if c == j0 {
            result *= c
        }
        else {
            result *= c + 1
        }
        //right
        c = (j0+1..width)
            .take_while(|&j| self.trees[[i0,j]].height < height_hiding_tree)
            .count();
        if c == (width - (j0 + 1)) {
            result *= c
        }
        else {
            result *= c + 1
        }
        result
    }
}
pub fn day8() {
    let mut file = File::open("./inputs/input_day8.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let length_size = data.split('\n').count();
    let mut array: Array2<Tree> = Array2::default((length_size,length_size));
    data.split('\n')
        .zip(array.axis_iter_mut(Axis(0)))
        .for_each(|(u,mut row)| u.chars().zip(row.iter_mut())
            .for_each(|(c,tree)| *tree = Tree::make_tree(c.to_digit(10).unwrap())));
    let mut trees = Trees { trees: array };
    (0..length_size)
        .for_each(|i| (0..length_size)
            .for_each(|j| trees.set_hiding(i,j)));
    let nb_visible = trees.trees.iter()
        .filter(|t| t.is_visible_from_outside())
        .count();
    println!("Solution 1 : {:?}",nb_visible);
    let v = (0..length_size)
        .cartesian_product(0..length_size)
        .map(|(i,j)| trees.compute_scenic_score(i,j))
        .max();
    println!("Solution 2 : {:?}",v.unwrap());

}