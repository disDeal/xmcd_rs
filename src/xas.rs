pub fn foo() {
    println!("Hello");
}

use peroxide::numerical::interp::lagrange_polynomial;
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
    fit_preedge: Vec<f64>,
    ene: Vec<f64>,
    energy: Vec<f64>,
    mui: Vec<f64>,
    mu: Vec<f64>,
    e0: f64,
}

impl Xas {
    pub fn new<R>(input: R) -> Result<Xas, Error>
    where
        R: io::BufRead,
    {
        let step = 0.1;
        let fit_preedge = Vec::new();

        let (ene, i0, i1) = Xas::load_from_file(input)?;
        assert!(ene.len() == i0.len() && i0.len() == i1.len());
        let size = ene.len();
        let mu = (0..size).map(|i| i1[i] / i0[i]).collect::<Vec<_>>();

        let start = ene[0].round();
        let stop = ene[size - 1].round();
        let num = ((stop - start) / step) as u32 + 1;
        let energy = peroxide::linspace!(start, stop, num);

        let mui = Xas::interpolate(ene.clone(), mu.clone(), energy.clone());
        let e0 = Xas::find_max_energy(mui.clone(), &energy)?;

        Ok(Xas {
            ene,

            mu,
            energy,
            mui,
            e0,

            fit_preedge,
        })
    }

    pub fn get_elem(&self) -> ! {
        unimplemented!()
    }

    fn interpolate(x: Vec<f64>, y: Vec<f64>, energy: Vec<f64>) -> Vec<f64> {
        let interp_func = lagrange_polynomial(x, y);
        interp_func.eval_vec(energy)
    }

    fn find_max_energy(array: Vec<f64>, energy: &[f64]) -> Result<f64, Error> {
        let muidiff = array.windows(2).map(|x| x[1] - x[0]);
        let mut iter = muidiff.enumerate();
        let init = iter.next().ok_or("Need at least one input").unwrap();
        let max_index = iter
            .try_fold(init, |acc, x| {
                let cmp = x.1.partial_cmp(&acc.1)?;
                let max = if let std::cmp::Ordering::Greater = cmp {
                    x
                } else {
                    acc
                };
                Some(max)
            })
            .unwrap()
            .0;
        Ok(energy[max_index])
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
