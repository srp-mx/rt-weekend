fn main() {
    // Image
    const IMAGE_WIDTH:i32 = 256;
    const IMAGE_HEIGHT:i32 = 256;

    // Render
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\nScanlines remaining: {}\n", j+1);
        for i in 0..IMAGE_WIDTH {
            let r:f64 = (i as f64) / ((IMAGE_WIDTH-1) as f64);
            let g:f64 = (j as f64) / ((IMAGE_HEIGHT-1) as f64);
            let b:f64 = 0.25f64;

            let ir:i32 = (255.999f64 * r) as i32;
            let ig:i32 = (255.999f64 * g) as i32;
            let ib:i32 = (255.999f64 * b) as i32;

            print!("{ir} {ig} {ib}\n");
        }
    }
    eprint!("\nDone.\n");
}
