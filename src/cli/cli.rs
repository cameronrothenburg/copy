use crate::cli::copy_args::{parse_args, CopyArgs};
use rayon::prelude::*;
use std::borrow::Borrow;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub struct Cli {
    args: CopyArgs,
    input_path: PathBuf,
    output_path: PathBuf,
    files_vec: Vec<PathBuf>,
}

impl Cli {
    pub fn new(env_args: Vec<String>) -> Cli {
        Cli {
            args: parse_args(env_args).expect("Usage: <input> <output>"),
            input_path: PathBuf::from(""),
            output_path: PathBuf::from(""),
            files_vec: Vec::new(),
        }
    }
    pub fn execute(&mut self) {
        self.create_path_buff();
        let input_path = self.input_path.as_path().to_owned();
        self.scan_dir(input_path.borrow())
            .expect("TODO: panic message");
        if self.files_vec.len() != 0 {
            self.copy()
        }
    }

    fn create_path_buff(&mut self) {
        self.input_path = PathBuf::from(self.args.copy_from.clone());
        self.output_path = PathBuf::from(self.args.copy_to.clone());
        self.validate_input_path()
    }

    fn validate_input_path(&self) {
        if !self.input_path.exists() {
            panic!("Input path does not exist!");
        }
    }

    fn scan_dir(&mut self, dir: &Path) -> io::Result<()> {
        // Run through all the files in the input path, add full path to files_vec.
        // If folder enter folder and call back

        // ./test/input/a/a.txt ./test/output/a/a.txt
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.scan_dir(&path)?;
                } else {
                    self.files_vec.push(entry.path());
                }
            }
        } else {
            self.files_vec.push(dir.to_path_buf());
        }

        Ok(())
    }
    fn copy(&self) {
        // Only works with directory's
        self.files_vec.clone().into_par_iter().for_each(|entry| {
            let copy_string = entry
                .as_path()
                .to_str()
                .unwrap()
                .replace::<&str>(self.args.copy_from.borrow(), "");
            let mut output_path_string = self
                .output_path
                .clone()
                .as_os_str()
                .to_str()
                .expect("")
                .to_owned();
            output_path_string.push_str(copy_string.as_str());
            let path_buffer = PathBuf::from(output_path_string);

            if let Some(parent) = path_buffer.parent() {
                fs::create_dir_all(parent).expect("TODO: panic message");
            };

            println!("Copying {:?} to {:?}", entry.as_path(), path_buffer);
            fs::copy(&entry, &path_buffer).expect("TODO: panic message");
        })
    }
}
