pub mod ppm;

fn main() {
    let filename = "out.ppm";
    let mut ppm = ppm::PPM::new(&filename, 100, 200, 256);

    for j in (0..ppm.height-1).rev() {
        for i in 0..ppm.width {
            let v1 = [
                (ppm.max as f32 * (i as f32 / ppm.width as f32)) as u32,
                (ppm.max as f32 * (j as f32 / ppm.height as f32)) as u32,
                52
            ];
            ppm.write_row(&v1);
        }
    }
}