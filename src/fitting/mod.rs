use nalgebra::{DMatrix, SVector};

pub enum Fitting {
    Polynoimal,
    Gaussian,
    Regression,
    RideRegression,
}

impl Fitting {
    fn slove<T>(fitting: Fitting) {
        match fitting {
            Fitting::Polynoimal => todo!(),
            Fitting::Gaussian => todo!(),
            Fitting::Regression => todo!(),
            Fitting::RideRegression => todo!(),
        }
    }
}

pub trait Fit {
    // fn resolve(&mut self, M: DMatrix<f32>) {}
}

/// 多项式逼近
/// n个点，多项式差值中，系数最高次等于n-1
pub struct PolynoimalFitting<'a, const D: usize> {
    data: &'a Vec<SVector<f32, D>>,
}

impl<'a, const D: usize> Fit for PolynoimalFitting<'a, D> {
    // fn resolve(&mut self) {}
}

impl<'a, const D: usize> PolynoimalFitting<'a, D> {
    pub fn new(data: &'a Vec<SVector<f32, D>>) -> Self {
        Self { data }
    }
    pub fn fitting(&mut self) {
        let r = self.data.len();
        let c = D;

        let A: DMatrix<f32> = DMatrix::zeros(r, r);
        let M = svector_2_matrix(self.data);
        // let (r, c) = self.data.shape();
        // let x_array = self.data.slice((0, c - 1), (0, r));
        // let y_array = self.data.slice((c, 0), (1, r));

        // let A: DMatrix<f32> = DMatrix::zeros(r, r);
        // let mut x_power = DMatrix::zeros(r, r).iter_mut().map(|_v| return 1.);

        // for i in 0..r {
        //     A.slice_mut((0, i), (i, c))
        // }
    }
}

/// 高斯拟合
///
pub struct GaussianFitting {}

impl Fit for GaussianFitting {
    // fn resolve(&mut self) {
    //     todo!()
    // }
}

/// 最小二乘回归
pub struct Regression {}
impl Fit for Regression {
    // fn resolve(&mut self) {
    //     todo!()
    // }
}

/// 岭回归
pub struct Ridegression {}
impl Fit for Ridegression {
    // fn resolve(&mut self) {
    //     todo!()
    // }
}

fn svector_2_matrix<'a, const D: usize>(v: &'a Vec<SVector<f32, D>>) -> DMatrix<f32> {
    let column = D;
    let row = v.len();
    let mut M = DMatrix::zeros(row, column);
    for index in 0..row {
        let sv = v[index].slice((0, 0), (0, column)).clone_owned();
        M.row_mut(index).copy_from(&sv)
    }

    // for (index, row) in M.row_iter_mut().enumerate() {
    //     let sv = &v[index];
    //     row = sv.clone();
    // }
    M
}
