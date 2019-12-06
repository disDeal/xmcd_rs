use gnuplot::*;

use nalgebra::DVector;
use rbf_interp::{Basis, Scatter};

use crate::{bail, Error, Reader};

#[derive(Debug)]
pub struct Xas {
    fit_preedge: Vec<f64>,
    ene: Vec<f64>,
    mu: Vec<f64>,
    pub energy: Vec<f64>,
    pub mui: Vec<f64>,
    pub e0: f64,
}

impl Xas {
    pub fn new<R>(input: R) -> Result<Xas, Error>
    where
        R: std::io::BufRead,
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
        let mut energy = (0..num)
            .map(|i| start + (i as f64 / size as f64) * (stop - start))
            .collect::<Vec<_>>();

        let mut mui = Xas::interpolate(ene.clone(), mu.clone(), energy.clone());
        let e0 = Xas::find_max_energy(mui.clone(), &energy)?;

        if mui.len() > mu.len() {
            let size = mui.len();
            let diff = mui.len() - mu.len();
            energy = energy[0..size - diff].to_vec();
            mui = mui[0..size - diff].to_vec();
        }

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
        let x = x
            .iter()
            .map(|&x| DVector::from_vec(vec![x]))
            .collect::<Vec<_>>();
        let y = y
            .iter()
            .map(|&x| DVector::from_vec(vec![x]))
            .collect::<Vec<_>>();

        let interp_func = Scatter::create(x, y, Basis::PolyHarmonic(2), 2);
        energy
            .iter()
            .map(|&x| interp_func.eval(DVector::from_vec(vec![f64::from(x)]))[0])
            .collect::<Vec<_>>()
    }

    fn find_max_energy(array: Vec<f64>, energy: &[f64]) -> Result<f64, Error> {
        // let muidiff = array.windows(2).map(|x| x[1] - x[0]);
        let mut iter = array.iter().enumerate();
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
            .expect("Cannot find max energy")
            .0;
        Ok(energy[max_index])
    }

    pub fn load_from_file<R>(mut input: R) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>), Error>
    where
        R: std::io::BufRead,
    {
        let mut ene = Vec::new();
        let mut i0 = Vec::new();
        let mut i1 = Vec::new();

        let mut buffer = String::new();
        while input.read_line(&mut buffer)? > 0 {
            let line = buffer
                .split(' ')
                .map(|s| s.trim().parse::<f64>().expect("Cannot parse to float"))
                .collect::<Vec<_>>();
            ene.push(line[0]);
            i0.push(line[1]);
            i1.push(line[2]);
            buffer.clear();
        }

        Ok((ene, i0, i1))
    }

    pub fn plot(&self) -> Result<(), Error> {
        let mut fg = Figure::new();
        fg.set_terminal("wxt size 1200,800", "out");
        let x = &self.ene;
        let x = x.iter2();
        let y = &self.mu;
        let y = y.iter2();

        let x2 = &self.energy;
        let x2 = x2.iter2();
        let y2 = &self.mui;
        let y2 = y2.iter2();

        fg.axes2d()
            .set_size(1.0, 0.7)
            .set_pos(0.0, 0.2)
            .points(
                x,
                y,
                &[
                    PointSymbol('o'),
                    Color("blue"),
                    BorderColor("red"),
                    LineWidth(2.0),
                ],
            )
            .lines(x2, y2, &[Color("red"), BorderColor("red")]);

        fg.show().unwrap();
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct BetterIterator<'l, T: 'l> {
    idx: usize,
    slice: &'l [T],
}

impl<'l, T: 'l> Iterator for BetterIterator<'l, T> {
    type Item = &'l T;
    fn next(&mut self) -> Option<&'l T> {
        let ret = self.slice.get(self.idx);
        self.idx += 1;
        ret
    }
}

pub trait BetterIteratorExt<'l, T> {
    fn iter2(self) -> BetterIterator<'l, T>;
}

impl<'l, T: 'l> BetterIteratorExt<'l, T> for &'l [T] {
    fn iter2(self) -> BetterIterator<'l, T> {
        BetterIterator {
            idx: 0,
            slice: self,
        }
    }
}
