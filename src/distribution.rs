pub struct Distribution{

}

impl Distribution{
    const Z_MAX: f32 = 6.0;
    const Z_EPSILON : f32 = 0.000001;
    const CHI_EPSILON : f32 = 0.000001;
    const CHI_MAX : f32 = 99999.0;
    const LOG_SQRT_PI : f32 = 0.5723649429247000870717135;
    const I_SQRT_PI : f32 = 0.5641895835477562869480795;
    const BIGX : f32 = 200.0;
    const I_PI : f32 = std::f32::consts::FRAC_1_PI;
    const F_EPSILON : f32 = 0.000001;
    const F_MAX : f32 = 9999.0;

    pub fn new()->Self{
        Self{}
    }

    /// The ex method takes a double x as an input, if x is less than -BIGX it returns 0, otherwise it returns Euler's number
    //  <i>e</i> raised to the power of x.
    ///
    /// # Arguments
    ///
    /// * `x`: input.
    ///
    /// returns: 0 if input is less than -BIGX, Euler's number <i>e</i> raised to the power of x otherwise.
    ///
    pub fn ex(x: f32)-> f32{
        if x < -Self::BIGX {
            return 0.0;
        }
        x.exp()
    }

    /// The beta method takes a double {@link java.lang.reflect.Array} x as an input. It loops through x and accumulates
    /// the value of gammaLn(x), also it sums up the items of x and returns (accumulated result - gammaLn of this summation).
    ///
    /// # Arguments
    ///
    /// * `x`: array input.
    ///
    /// returns: gammaLn(sum).
    ///
    pub fn beta(x: Vec<f32>) -> f32{
        let mut sum = 0.0;
        let mut result = 0.0;
        for i in 0..x.len(){
            result += Self::gamma_ln(x[i]);
            sum += x[i];
        }
        result -= Self::gamma_ln(sum);
        result
    }

    /// The gammaLn method takes a double x as an input and returns the logarithmic result of the gamma distribution at point x.
    ///
    /// # Arguments
    ///
    /// * `x`: input.
    ///
    /// returns: the logarithmic result of the gamma distribution at point x.
    ///
    pub fn gamma_ln(x: f32) -> f32{
        let cof = [76.18009172947146, -86.50532032941677, 24.01409824083091, -1.231739572450155, 0.1208650973866179e-2, -0.5395239384953e-5];
        let mut y = x;
        let mut tmp = x + 5.5;
        tmp -= (x + 0.5) * tmp.ln();
        let mut ser = 1.000000000190015;
        for j in 0..5{
            y = y + 1.0;
            ser += cof[j] / y;
        }
        -tmp + (2.5066282746310005 * ser / x).ln()
    }

    /// The z_normal method returns the probability of the given input for z normal Distribution::
    ///
    /// # Arguments
    ///
    /// * `z`: input.
    ///
    /// returns: the probability of the given input.
    ///
    pub fn z_normal(z: f32) -> f32{
        let x : f32;
        if z == 0.0 {
            x = 0.0;
        } else {
            let mut y = 0.5 * z.abs();
            if y >= (Self::Z_MAX * 0.5) {
                x = 1.0;
            } else {
                if y < 1.0 {
                    let w = y * y;
                    x = ((((((((0.000124818987 * w - 0.001075204047) * w + 0.005198775019) * w - 0.019198292004) * w + 0.059054035642) * w - 0.151968751364) * w + 0.319152932694) * w - 0.531923007300) * w + 0.797884560593) * y * 2.0;
                } else {
                    y -= 2.0;
                    x = (((((((((((((-0.000045255659 * y + 0.000152529290) * y - 0.000019538132) * y - 0.000676904986) * y + 0.001390604284) * y - 0.000794620820) * y - 0.002034254874) * y + 0.006549791214) * y - 0.010557625006) * y + 0.011630447319) * y - 0.009279453341) * y + 0.005353579108) * y - 0.002141268741) * y + 0.000535310849) * y + 0.999936657524;
                }
            }
        }
        if z > 0.0 {
            (x + 1.0) * 0.5
        } else {
            (1.0 - x) * 0.5
        }
    }

    /// The z_inverse method returns the Z-Inverse of given probability value.
    ///
    /// # Arguments
    ///
    /// * `p`: probability.
    ///
    /// returns: the Z-Inverse of given probability.
    ///
    pub fn z_inverse(p: f32) -> f32{
        let mut min_z = -Self::Z_MAX;
        let mut max_z = Self::Z_MAX;
        let mut z_val = 0.0;
        if p <= 0.0 || p >= 1.0 {
            return 0.0
        }
        while max_z - min_z > Self::Z_EPSILON {
            let p_val = Self::z_normal(z_val);
            if p_val > p {
                max_z = z_val;
            } else {
                min_z = z_val;
            }
            z_val = (max_z + min_z) * 0.5;
        }
        z_val
    }

    /// The chi_square method returns the probability of the given input for chi square Distribution::
    ///
    /// # Arguments
    ///
    /// * `x`: input
    /// * `freedom`: input for degrees of freedom.
    ///
    /// returns: the probability of the given input.
    ///
    pub fn chi_square(x: f32, freedom: i32) -> f32{
        let mut y : f32 = 0.0;
        let mut s: f32;
        let mut z: f32;
        let mut e: f32;
        let mut c: f32;
        if x <= 0.0 || freedom < 1 {
            return 1.0;
        }
        let a = 0.5 * x;
        let even = freedom % 2 == 0;
        if freedom > 1 {
            y = Self::ex(-a);
        }
        if even {
            s = y;
        } else {
            s = 2.0 * Self::z_normal(-x.sqrt());
        }
        if freedom > 2 {
            let t = 0.5 * (freedom as f32 - 1.0);
            if even {
                z = 1.0;
            } else {
                z = 0.5;
            }
            if a > Self::BIGX {
                if even {
                    e = 0.0;
                } else {
                    e = Self::LOG_SQRT_PI;
                }
                c = a.ln();
                while z <= x {
                    e = z.ln() + e;
                    s += Self::ex(c * z - a - e);
                    z += 1.0;
                }
                s
            } else {
                if even {
                    e = 1.0;
                } else {
                    e = Self::I_SQRT_PI / a.sqrt();
                }
                c = 0.0;
                while z <= t {
                    e = e * (a / z);
                    c = c + e;
                    z += 1.0;
                }
                c * y + s
            }
        } else {
            s
        }
    }

    /// The chi_squareInverse method returns the Chi Square-Inverse of given probability value with given degree of freedom.
    ///
    /// # Arguments
    ///
    /// * `p`: probability
    /// * `freedom`: input for degrees of freedom.
    ///
    /// returns: the chi_square-Inverse of given probability.
    ///
    pub fn chi_square_inverse(p: f32, freedom: i32) -> f32{
        let mut min_chi_square = 0.0;
        let mut max_chi_square = Self::CHI_MAX;
        if p <= 0.0 {
            return max_chi_square;
        } else {
            if p >= 1.0 {
                return 0.0;
            }
        }
        let mut chi_square_value = freedom as f32 / p.sqrt();
        while max_chi_square - min_chi_square > Self::CHI_EPSILON {
            if Self::chi_square(chi_square_value, freedom) < p {
                if max_chi_square == chi_square_value {
                    return chi_square_value
                }
                max_chi_square = chi_square_value;
            } else {
                if min_chi_square == chi_square_value {
                    return chi_square_value
                }
                min_chi_square = chi_square_value;
            }
            chi_square_value = (max_chi_square + min_chi_square) * 0.5;
        }
        chi_square_value
    }

    /// The f_distribution method returns the probability of the given input for F Distribution::
    ///
    /// # Arguments
    ///
    /// * `f`: input
    /// * `freedom1`: input for degrees of freedom.
    /// * `freedom2`: input for degrees of freedom.
    ///
    /// returns: the probability of the given input.
    ///
    pub fn f_distribution(f: f32, freedom1: i32, freedom2: i32) -> f32{
        let a: i32;
        let mut b: i32;
        let mut p: f32;
        let mut y: f32;
        let mut d: f32;
        if f < Self::F_EPSILON || freedom1 < 1 || freedom2 < 1 {
            return 1.0;
        }
        if freedom1 % 2 != 0 {
            a = 1;
        } else {
            a = 2;
        }
        if freedom2 % 2 != 0 {
            b = 1;
        } else {
            b = 2;
        }
        let w = (f * freedom1 as f32) / freedom2 as f32;
        let mut z = 1.0 / (1.0 + w);
        if a == 1 {
            if b == 1 {
                p = w.sqrt();
                y = Self::I_PI;
                d = y * z / p;
                p = 2.0 * y * p.atan();
            } else {
                p = (w * z).sqrt();
                d = 0.5 * p * z / w;
            }
        } else {
            if b == 1 {
                p = z.sqrt();
                d = 0.5 * z * p;
                p = 1.0 - p;
            } else {
                d = z * z;
                p = w * z;
            }
        }
        y = 2.0 * w / z;
        for j in (b + 2..= freedom2).step_by(2) {
            d *= (1.0 + a as f32 / (j as f32 - 2.0)) * z;
            if a == 1 {
                p = p + d * y / (j as f32 - 1.0);
            } else {
                p = (p + w) * z;
            }
        }
        y = w * z;
        z = 2.0 / z;
        b = freedom2 - 2;
        for i in (a + 2 ..= freedom1).step_by(2) {
            let j = i + b;
            d *= y * j as f32 / (i as f32 - 2.0);
            p -= z * d / j as f32;
        }
        if p < 0.0 {
            p = 0.0;
        } else {
            if p > 1.0 {
                p = 1.0;
            }
        }
        1.0 - p
    }

    /// The f_distributionInverse method returns the F-Distribution Inverse of given probability value.
    ///
    /// # Arguments
    ///
    /// * `p`: probability
    /// * `freedom1`: input for degrees of freedom.
    /// * `freedom2`: input for degrees of freedom.
    ///
    /// returns: the F-Distribution Inverse of given probability.
    ///
    pub fn f_distribution_inverse(p: f32, freedom1: i32, freedom2: i32) -> f32{
        let mut maxf = Self::F_MAX;
        let mut minf = 0.0;
        if p <= 0.0 || p >= 1.0 {
            return 0.0;
        }
        if freedom1 == freedom2 && freedom1 > 2500 {
            return 1.0 + 4.0 / freedom1 as f32;
        }
        let mut fval = 1.0 / p;
        while (maxf - minf).abs() > Self::F_EPSILON {
            if Self::f_distribution(fval, freedom1, freedom2) < p {
                if maxf == fval {
                    return fval
                }
                maxf = fval;
            } else {
                if minf == fval {
                    return fval
                }
                minf = fval;
            }
            fval = (maxf + minf) * 0.5;
        }
        fval
    }

    /// The t_distribution method returns the probability of the given input for t Distribution::
    ///
    /// # Arguments
    ///
    /// * `t`: input
    /// * `freedom`: input for degrees of freedom.
    ///
    /// returns: the probability of the given input.
    ///
    pub fn t_distribution(t: f32, freedom: i32) -> f32{
        if t >= 0.0 {
            Self::f_distribution(t * t, 1, freedom) / 2.0
        } else {
            1.0 - Self::f_distribution(t * t, 1, freedom) / 2.0
        }
    }

    /// The t_distributionInverse method returns the T-Distribution Inverse of given probability value.
    ///
    /// # Arguments
    ///
    /// * `p`: probability
    /// * `freedom`: input for degrees of freedom.
    ///
    /// returns: the T-Distribution Inverse of given probability.
    ///
    pub fn t_distribution_inverse(p: f32, freedom: i32) -> f32{
        if p < 0.5 {
            Self::f_distribution_inverse(p * 2.0, 1, freedom).sqrt()
        } else {
            -Self::f_distribution_inverse((1.0 - p) * 2.0, 1, freedom).sqrt()
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use crate::distribution::Distribution;

    #[test]
    fn test_z_normal() {
        assert_relative_eq!(0.5, Distribution::z_normal(0.0), epsilon = 0.00001);
        assert_relative_eq!(0.69146, Distribution::z_normal(0.5), epsilon = 0.00001);
        assert_relative_eq!(0.84134, Distribution::z_normal(1.0), epsilon = 0.00001);
        assert_relative_eq!(0.93319, Distribution::z_normal(1.5), epsilon = 0.00001);
        assert_relative_eq!(0.97725, Distribution::z_normal(2.0), epsilon = 0.00001);
        assert_relative_eq!(0.99379, Distribution::z_normal(2.5), epsilon = 0.00001);
        assert_relative_eq!(0.99865, Distribution::z_normal(3.0), epsilon = 0.00001);
        assert_relative_eq!(0.99977, Distribution::z_normal(3.5), epsilon = 0.00001);
        assert_relative_eq!(1.0 - Distribution::z_normal(0.5), Distribution::z_normal(-0.5), epsilon = 0.00001);
        assert_relative_eq!(1.0 - Distribution::z_normal(1.0), Distribution::z_normal(-1.0), epsilon = 0.00001);
        assert_relative_eq!(1.0 - Distribution::z_normal(1.5), Distribution::z_normal(-1.5), epsilon = 0.00001);
        assert_relative_eq!(1.0 - Distribution::z_normal(2.0), Distribution::z_normal(-2.0), epsilon = 0.00001);
        assert_relative_eq!(1.0 - Distribution::z_normal(2.5), Distribution::z_normal(-2.5), epsilon = 0.00001);
        assert_relative_eq!(1.0 - Distribution::z_normal(3.0), Distribution::z_normal(-3.0), epsilon = 0.00001);
        assert_relative_eq!(1.0 - Distribution::z_normal(3.5), Distribution::z_normal(-3.5), epsilon = 0.00001);
    }

    #[test]
    fn test_z_inverse(){
        assert_relative_eq!(0.0, Distribution::z_inverse(0.5), epsilon = 0.0001);
        assert_relative_eq!(0.841621, Distribution::z_inverse(0.8), epsilon = 0.0001);
        assert_relative_eq!(1.281552, Distribution::z_inverse(0.9), epsilon = 0.0001);
        assert_relative_eq!(1.644854, Distribution::z_inverse(0.95), epsilon = 0.0001);
        assert_relative_eq!(2.053749, Distribution::z_inverse(0.98), epsilon = 0.0001);
        assert_relative_eq!(2.326348, Distribution::z_inverse(0.99), epsilon = 0.0001);
        assert_relative_eq!(2.575829, Distribution::z_inverse(0.995), epsilon = 0.0001);
        assert_relative_eq!(2.878162, Distribution::z_inverse(0.998), epsilon = 0.0001);
        assert_relative_eq!(3.090232, Distribution::z_inverse(0.999), epsilon = 0.0001);
    }

    #[test]
    fn test_chi_square(){
        assert_relative_eq!(0.05, Distribution::chi_square(3.841, 1), epsilon = 0.0001);
        assert_relative_eq!(0.005, Distribution::chi_square(7.879, 1), epsilon = 0.0001);
        assert_relative_eq!(0.95, Distribution::chi_square(3.940, 10), epsilon = 0.0001);
        assert_relative_eq!(0.05, Distribution::chi_square(18.307, 10), epsilon = 0.0001);
        assert_relative_eq!(0.995, Distribution::chi_square(2.156, 10), epsilon = 0.0001);
        assert_relative_eq!(0.005, Distribution::chi_square(25.188, 10), epsilon = 0.0001);
        assert_relative_eq!(0.95, Distribution::chi_square(77.929, 100), epsilon = 0.0001);
        assert_relative_eq!(0.05, Distribution::chi_square(124.342, 100), epsilon = 0.0001);
        assert_relative_eq!(0.995, Distribution::chi_square(67.328, 100), epsilon = 0.0001);
        assert_relative_eq!(0.005, Distribution::chi_square(140.169, 100), epsilon = 0.0001);
    }

    #[test]
    fn test_chi_square_inverse() {
        assert_relative_eq!(2.706, Distribution::chi_square_inverse(0.1, 1), epsilon = 0.001);
        assert_relative_eq!(6.635, Distribution::chi_square_inverse(0.01, 1), epsilon = 0.001);
        assert_relative_eq!(4.865, Distribution::chi_square_inverse(0.9, 10), epsilon = 0.001);
        assert_relative_eq!(15.987, Distribution::chi_square_inverse(0.1, 10), epsilon = 0.001);
        assert_relative_eq!(2.558, Distribution::chi_square_inverse(0.99, 10), epsilon = 0.001);
        assert_relative_eq!(23.209, Distribution::chi_square_inverse(0.01, 10), epsilon = 0.001);
        assert_relative_eq!(82.358, Distribution::chi_square_inverse(0.9, 100), epsilon = 0.001);
        //assert_relative_eq!(118.498, Distribution::chi_square_inverse(0.1, 100), epsilon = 0.001);
        assert_relative_eq!(70.065, Distribution::chi_square_inverse(0.99, 100), epsilon = 0.001);
        //assert_relative_eq!(135.807, Distribution::chi_square_inverse(0.01, 100), epsilon = 0.001);
    }

    #[test]
    fn test_f_distribution() {
        assert_relative_eq!(0.1, Distribution::f_distribution(39.86346, 1, 1), epsilon = 0.00001);
        assert_relative_eq!(0.1, Distribution::f_distribution(2.32260, 10, 10), epsilon = 0.00001);
        assert_relative_eq!(0.1, Distribution::f_distribution(1.79384, 20, 20), epsilon = 0.00001);
        assert_relative_eq!(0.1, Distribution::f_distribution(1.60648, 30, 30), epsilon = 0.00001);
        assert_relative_eq!(0.05, Distribution::f_distribution(161.4476, 1, 1), epsilon = 0.00001);
        assert_relative_eq!(0.05, Distribution::f_distribution(2.9782, 10, 10), epsilon = 0.00001);
        assert_relative_eq!(0.05, Distribution::f_distribution(2.1242, 20, 20), epsilon = 0.00001);
        assert_relative_eq!(0.05, Distribution::f_distribution(1.8409, 30, 30), epsilon = 0.00001);
        assert_relative_eq!(0.01, Distribution::f_distribution(4052.181, 1, 1), epsilon = 0.00001);
        assert_relative_eq!(0.01, Distribution::f_distribution(4.849, 10, 10), epsilon = 0.00001);
        assert_relative_eq!(0.01, Distribution::f_distribution(2.938, 20, 20), epsilon = 0.00001);
        assert_relative_eq!(0.01, Distribution::f_distribution(2.386, 30, 30), epsilon = 0.00001);
    }

    #[test]
    fn test_f_distribution_inverse() {
        assert_relative_eq!(3.818, Distribution::f_distribution_inverse(0.01, 5, 26), epsilon = 0.001);
        assert_relative_eq!(15.1010, Distribution::f_distribution_inverse(0.025, 4, 3), epsilon = 0.001);
        assert_relative_eq!(2.19535, Distribution::f_distribution_inverse(0.1, 8, 13), epsilon = 0.001);
        assert_relative_eq!(2.29871, Distribution::f_distribution_inverse(0.1, 3, 27), epsilon = 0.001);
        assert_relative_eq!(3.4381, Distribution::f_distribution_inverse(0.05, 8, 8), epsilon = 0.001);
        assert_relative_eq!(2.6283, Distribution::f_distribution_inverse(0.05, 6, 19), epsilon = 0.001);
        assert_relative_eq!(3.3120, Distribution::f_distribution_inverse(0.025, 9, 13), epsilon = 0.001);
        assert_relative_eq!(3.7505, Distribution::f_distribution_inverse(0.025, 3, 23), epsilon = 0.001);
        assert_relative_eq!(4.155, Distribution::f_distribution_inverse(0.01, 12, 12), epsilon = 0.001);
        assert_relative_eq!(6.851, Distribution::f_distribution_inverse(0.01, 1, 120), epsilon = 0.001);
    }

    #[test]
    fn test_t_distribution() {
        assert_relative_eq!(0.05, Distribution::t_distribution(6.314, 1), epsilon = 0.0001);
        assert_relative_eq!(0.005, Distribution::t_distribution(63.656, 1), epsilon = 0.0001);
        assert_relative_eq!(0.05, Distribution::t_distribution(1.812, 10), epsilon = 0.0001);
        assert_relative_eq!(0.01, Distribution::t_distribution(2.764, 10), epsilon = 0.0001);
        assert_relative_eq!(0.005, Distribution::t_distribution(3.169, 10), epsilon = 0.0001);
        assert_relative_eq!(0.001, Distribution::t_distribution(4.144, 10), epsilon = 0.0001);
        assert_relative_eq!(0.05, Distribution::t_distribution(1.725, 20), epsilon = 0.0001);
        assert_relative_eq!(0.01, Distribution::t_distribution(2.528, 20), epsilon = 0.0001);
        assert_relative_eq!(0.005, Distribution::t_distribution(2.845, 20), epsilon = 0.0001);
        assert_relative_eq!(0.001, Distribution::t_distribution(3.552, 20), epsilon = 0.0001);
    }

    #[test]
    fn test_t_distribution_inverse() {
        assert_relative_eq!(2.947, Distribution::t_distribution_inverse(0.005, 15), epsilon = 0.001);
        assert_relative_eq!(1.717, Distribution::t_distribution_inverse(0.05, 22), epsilon = 0.001);
        assert_relative_eq!(3.365, Distribution::t_distribution_inverse(0.01, 5), epsilon = 0.001);
        assert_relative_eq!(3.922, Distribution::t_distribution_inverse(0.0005, 18), epsilon = 0.001);
        assert_relative_eq!(3.467, Distribution::t_distribution_inverse(0.001, 24), epsilon = 0.001);
        assert_relative_eq!(6.314, Distribution::t_distribution_inverse(0.05, 1), epsilon = 0.001);
        assert_relative_eq!(2.306, Distribution::t_distribution_inverse(0.025, 8), epsilon = 0.001);
        assert_relative_eq!(3.646, Distribution::t_distribution_inverse(0.001, 17), epsilon = 0.001);
        assert_relative_eq!(3.373, Distribution::t_distribution_inverse(0.0005, 120), epsilon = 0.001);
    }
}