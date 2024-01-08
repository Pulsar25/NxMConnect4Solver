use std::collections::HashSet;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::{Write, BufWriter,BufRead, BufReader};
use std::sync::{Arc, Mutex};
extern crate crossbeam;
use crossbeam::channel::unbounded;
use std::fs::File;
