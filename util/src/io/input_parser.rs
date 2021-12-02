//! Input Parser for AoC.
//!
//! This module only supports parsing input from plain text
//! file with format of one entry per line followed by a new
//! line character.
//! Output will be a vector with passed type parameter if
//! given a valid file path.
//! Each indices of the vector will contain each line of content
//! given in the file.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct InputParser<TypeParamPlaceholder> {
    file_path: String,
    phantom_data: PhantomData<TypeParamPlaceholder>,
}

impl<TypeParamPlaceholder> InputParser<TypeParamPlaceholder>
    where
        TypeParamPlaceholder: std::str::FromStr,
        <TypeParamPlaceholder as std::str::FromStr>::Err: std::fmt::Debug,
{
    pub fn data<T>(file_path: &str) -> Vec<T>
        where
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug, {
        let file = File::open(file_path).expect("File not found.");
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|line| line.unwrap().parse::<T>().unwrap())
            .collect::<Vec<T>>()
    }
}
