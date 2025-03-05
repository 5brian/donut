use std::f64::consts::PI;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;
    let chars = ".,-~:;=!*#$@".as_bytes();

    loop {
        let mut output = vec![' '; 1760];
        let mut z_buf = vec![0.0; 1760];
        let (sin_a, cos_a) = a.sin_cos();
        let (sin_b, cos_b) = b.sin_cos();

        let mut theta: f64 = 0.0;
        while theta < 2.0 * PI {
            let (sin_theta, cos_theta) = theta.sin_cos();
            let mut phi: f64 = 0.0;
            while phi < 2.0 * PI {
                let (sin_phi, cos_phi) = phi.sin_cos();

                // torus coordinates
                let h = cos_theta + 2.0;
                let d = 1.0 / (sin_phi * h * sin_a + sin_theta * cos_a + 5.0);
                let t = sin_phi * h * cos_a - sin_theta * sin_a;

                // screen coordinates
                let x = (40.0 + 30.0 * d * (cos_phi * h * cos_b - t * sin_b)) as usize;
                let y = (12.0 + 15.0 * d * (cos_phi * h * sin_b + t * cos_b)) as usize;

                // luminance calculation
                let l = 8.0
                    * ((sin_theta * sin_a - sin_phi * cos_theta * cos_a) * cos_b
                        - sin_phi * cos_theta * sin_a
                        - sin_theta * cos_a
                        - cos_phi * cos_theta * sin_b);

                if y < 22 && x < 80 && d > z_buf[x + y * 80] {
                    z_buf[x + y * 80] = d;
                    output[x + y * 80] = if l > 0.0 {
                        chars[l as usize % chars.len()]
                    } else {
                        chars[0]
                    } as char;
                }
                phi += 0.02;
            }
            theta += 0.07;
        }

        print!("\x1b[H");
        for (i, c) in output.iter().enumerate() {
            if i % 80 == 0 {
                println!();
            }
            print!("{}", c);
        }

        io::stdout().flush().unwrap();
        a += 0.04;
        b += 0.02;
        thread::sleep(Duration::from_millis(30));
    }
}
