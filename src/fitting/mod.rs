use nalgebra::{vector, DMatrix, DVector, OVector, Point2, SVector};

pub enum Fitting {
    PolynoimalInterpolation,
    GaussianInterpolation,
    Regression,
    RideRegression,
}

impl Fitting {
    fn slove<T>(fitting: Fitting) {
        match fitting {
            Fitting::PolynoimalInterpolation => todo!(),
            Fitting::GaussianInterpolation => todo!(),
            Fitting::Regression => todo!(),
            Fitting::RideRegression => todo!(),
        }
    }
}

pub trait Fit {
    /// 求解方程组
    /// 如果方程组是超定的 那么使用SVD求解最小特征
    /// 如果方程组是欠定的 , 过拟合
    fn resolve_equation(&mut self, A: DMatrix<f32>, b: DMatrix<f32>) -> DVector<f32> {
        let shape = A.shape();
        if shape.0 >= shape.1 {
            let svd = A.try_svd(true, true, 0.0001, 1000).unwrap();
            let v_t = svd.v_t.unwrap();
            let u = svd.u.unwrap();
            let a = v_t.transpose() * &svd.singular_values * &u.transpose();
            let v = (a * &b).data;
            let v = v.as_vec().to_owned();
            return DVector::from_vec(v);
        } else {
        }
        todo!()
    }

    fn draw_series();
}

/// 多项式逼近
/// n个点，多项式差值中，系数最高次等于n-1
pub struct PolynoimalInterpolation<'a> {
    data: &'a Vec<Point2<f32>>,
}

impl<'a> Fit for PolynoimalInterpolation<'a> {
    fn draw_series() {
        todo!()
    }
}

impl<'a> PolynoimalInterpolation<'a> {
    pub fn new(data: &'a Vec<Point2<f32>>) -> Self {
        Self { data }
    }
    pub fn fitting(&mut self) {
        let r = self.data.len();
        let data_matrix = vector_point_2_matirx(self.data);
        let mut A: DMatrix<f32> = DMatrix::zeros(r, r);
        let mut x_array = data_matrix.slice((0, 0), (r, 0)).clone_owned();
        let y = data_matrix.slice((0, 1), (r, 1)).clone_owned();

        let mut x_power = x_array.clone_owned();
        for i in 0..r {
            if i == 0 {
                A.fill_column(0, 1.);
            } else {
                A.column_mut(i)
                    .copy_from(&x_power.slice((0, 0), (r, 1)).clone_owned());
                x_power = x_power * &x_array;
            }
        }
    }
}

/// 高斯拟合
///
pub struct GaussianInterpolation {}

impl Fit for GaussianInterpolation {
    fn draw_series() {
        todo!()
    }
    // fn resolve(&mut self) {
    //     todo!()
    // }
}

/// 最小二乘回归
pub struct Regression {}
impl Fit for Regression {
    fn draw_series() {
        todo!()
    }
    // fn resolve(&mut self) {
    //     todo!()
    // }
}

/// 岭回归
pub struct Ridegression {}
impl Fit for Ridegression {
    fn draw_series() {
        todo!()
    }
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

fn vector_point_2_matirx(v: &Vec<Point2<f32>>) -> DMatrix<f32> {
    let r = v.len();
    let c = 2 as usize;
    let mut M = DMatrix::zeros(r, c);
    for ri in 0..r {
        let point = &v[ri];
        M.row_mut(ri).copy_from_slice(&[point.x, point.y]);
    }
    M
}
