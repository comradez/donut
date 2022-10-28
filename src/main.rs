use std::{f64::consts::PI, process::exit};

const CHARS: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];
const HEIGHT: usize = 50;
const WIDTH: usize = 50;

struct Donut<const H: usize, const W: usize> {
    r1: f64,
    r2: f64,
    k1: f64,
    k2: f64,
    pub frame: [[char; W]; H],
    z_buf: [[f64; W]; H],
}

impl<const H: usize, const W: usize> Donut<H, W> {
    pub fn new(r1: f64, r2: f64, k2: f64) -> Self {
        let k1 = W as f64 * k2 * 3. / (8. * (r1 + r2));
        let frame: [[char; W]; H] = [[' '; W]; H];
        let z_buf: [[f64; W]; H] = [[0.; W]; H];
        Self {
            r1,
            r2,
            k1,
            k2,
            frame,
            z_buf,
        }
    }

    pub fn render(&mut self, a: f64, b: f64) {
        self.frame = [[' '; W]; H];
        self.z_buf = [[0.; W]; H];
        let (sin_a, cos_a) = a.sin_cos();
        let (sin_b, cos_b) = b.sin_cos();
        for theta in 0..314 {
            let theta = theta as f64 * 2. * PI / 314f64;
            let (sin_theta, cos_theta) = theta.sin_cos();
            for phi in 0..628 {
                let phi = phi as f64 * 2. * PI / 628f64;
                let (sin_phi, cos_phi) = phi.sin_cos();

                let (circle_x, circle_y) = (self.r2 + self.r1 * cos_theta, self.r1 * sin_theta);
                let x = circle_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi)
                    - circle_y * cos_a * sin_b;
                let y = circle_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi)
                    + circle_y * cos_a * cos_b;
                let z = self.k2 + cos_a * circle_x * sin_phi + circle_y * sin_a;

                let x_projection = (W as isize / 2 + (self.k1 * x / z) as isize) as usize;
                let y_projection = (H as isize / 2 + (self.k1 * y / z) as isize) as usize;

                let luminance =
                    cos_phi * cos_theta * sin_b - cos_a * cos_theta * sin_phi - sin_a * sin_theta
                        + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi);
                if x_projection < W
                    && y_projection < H
                    && luminance > 0.
                    && 1. / z > self.z_buf[y_projection][x_projection]
                {
                    self.z_buf[y_projection][x_projection] = 1. / z;
                    self.frame[y_projection][x_projection] = CHARS[(luminance * 8.) as usize];
                }
            }
        }
    }
}

fn main() {
    print!("\x1b[2J");
    print!("\x1b[?25l");

    ctrlc::set_handler(move || {
        print!("\x1b[?25h");
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let mut donut: Donut<HEIGHT, WIDTH> = Donut::new(1., 2., 5.);
    let (mut a, mut b) = (1., 1.);

    loop {
        print!("\x1b[H");
        donut.render(a, b);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", donut.frame[y][x]);
            }
            println!();
        }
        a += 0.07;
        b += 0.03;
        std::thread::sleep(std::time::Duration::from_millis(1000 / 30));
    }
}
