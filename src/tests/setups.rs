use {
    crate::{
        tests::auxiliary::{
            random_vector3,
            random_angle,
            rot,
            random_norm_vector3
        },

        subproblems::subproblem1
    },

    nalgebra::Vector3,
};

pub trait Setup {
    fn name(&self) -> &'static str;
    fn is_at_min(&self) -> bool;
    fn error(&self) -> f64;

    fn setup(&mut self);
    fn setup_ls(&mut self);
    fn run(&mut self);
}

pub struct Subproblem1Setup {
    p1: Vector3<f64>,
    p2: Vector3<f64>,
    k: Vector3<f64>,

    theta: f64,
}

impl Subproblem1Setup {
    pub fn new() -> Self {
        Self {
            p1: Vector3::zeros(),
            p2: Vector3::zeros(),
            k: Vector3::zeros(),

            theta: 0.0,
        }
    }

    fn calculate_error(&self, theta: f64) -> f64 {
        (self.p2 - rot(self.k, theta) * self.p1).norm()
    }
}

impl Setup for Subproblem1Setup {
    fn setup(&mut self) {
        self.p1 = random_vector3();
        self.k = random_norm_vector3();
        self.theta = random_angle();

        self.p2 = rot(self.k, self.theta) * self.p1;
    }

    fn is_at_min(&self) -> bool {
        const DELTA: f64 = 1e-12;

        let error = self.error();
        let error_check = error - DELTA;

        self.calculate_error(error + DELTA) >= error_check &&
        self.calculate_error(error - DELTA) >= error_check
    }

    fn error(&self) -> f64 {
        self.calculate_error(self.theta)
    }

    fn setup_ls(&mut self) {
        self.p1 = random_vector3();
        self.p2 = random_vector3();
        self.k = random_norm_vector3();
        self.theta = random_angle();
    }

    fn run(&mut self) {
        (self.theta, _) = subproblem1(self.p1, self.p2, self.k);
    }

    fn name(&self) -> &'static str {
        "Subproblem 1"
    }
}
