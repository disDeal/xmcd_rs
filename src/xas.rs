pub fn foo() {
    println!("Hello");
}

use peroxide::*;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::exit;

use structopt::StructOpt;

use crate::{bail, Error, Reader};

#[derive(Debug, StructOpt)]
struct Opt {
    /// Optional path to input file; if not supplied will read from stdin
    input: Option<PathBuf>,
}

pub struct Xas {
    step: f64,
    param: Vec<f64>,
    cov: Vec<f64>,
    fit_preedge: Vec<f64>,
    ene: Vec<f64>,
    i0: Vec<f64>,
    i1: Vec<f64>,
    energy: Vec<f64>,
    mui: Vec<f64>,
    mu: Vec<f64>,
    e0: f64,
}

impl Xas {
    pub fn new<R>(mut input: R) -> Result<Xas, Error>
    where
        R: io::BufRead,
    {
        let step = 0.1;
        let param = Vec::new();
        let cov = Vec::new();
        let fit_preedge = Vec::new();
        let (ene, i0, i1) = Xas::load_from_file(input)?;
        assert!(ene.len() == i0.len() && i0.len() == i1.len());
        let size = ene.len();
        let mu = (0..size).map(|i| i1[i] / i0[i]).collect::<Vec<_>>();
        let start = ene[0].round();
        let stop = ene[size - 1].round();
        let num = ((stop - start) / step) as u32 + 1;
        let energy = peroxide::linspace!(start, stop, num);
        let mui = Xas::interpolate(ene.clone(), mu.clone());
        let e0 = Xas::find_max_energy(mui.clone());
        Ok(Xas {
            step,
            ene,
            i0,
            i1,
            mu,
            energy,
            mui,
            e0,
            param,
            cov,
            fit_preedge,
        })
    }

    pub fn get_elem(&self) -> ! {
        unimplemented!()
    }

    fn interpolate(x: Vec<f64>, y: Vec<f64>) -> Vec<f64> {
        unimplemented!()
    }

    fn find_max_energy(array: Vec<f64>) -> f64 {
        unimplemented!()
    }

    pub fn load_from_file<R>(mut input: R) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>), Error>
    where
        R: io::BufRead,
    {
        let mut ene = Vec::new();
        let mut i0 = Vec::new();
        let mut i1 = Vec::new();

        let mut buffer = String::new();
        while input.read_line(&mut buffer)? > 0 {
            let line = buffer
                .split(' ')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect::<Vec<_>>();
            ene.push(line[0]);
            i0.push(line[1]);
            i1.push(line[2]);
            buffer.clear();
        }

        Ok((ene, i0, i1))
    }
}
