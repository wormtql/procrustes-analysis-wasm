mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-wasm!");
}

fn remove_translate(a: &mut Vec<f64>, shapes: i32, dots: i32) {
    for i in 0..shapes {
        let mut x_bar: f64 = 0.0;
        let mut y_bar: f64 = 0.0;
        for j in 0..dots {
            let val_x = a[(i * dots * 2 + j * 2) as usize];
            let val_y = a[(i * dots * 2 + j * 2 + 1) as usize];
            x_bar += val_x;
            y_bar += val_y;
        }
        x_bar /= dots as f64;
        y_bar /= dots as f64;
        for j in 0..dots {
            a[(i * dots * 2 + j * 2) as usize] -= x_bar;
            a[(i * dots * 2 + j * 2 + 1) as usize] -= y_bar;
        }
    }
}

fn remove_scale(a: &mut Vec<f64>, shapes: i32, dots: i32) {
    for i in 0..shapes {
        let mut s: f64 = 0.0;
        for j in 0..dots {
            let val_x = a[(i * dots * 2 + j * 2) as usize];
            let val_y = a[(i * dots * 2 + j * 2 + 1) as usize];
            s += val_x * val_x + val_y * val_y;
        }
        s = (s / dots as f64).sqrt();
        if s == 0.0 {
            continue;
        }
        for j in 0..dots {
            a[(i * dots * 2 + j * 2) as usize] /= s;
            a[(i * dots * 2 + j * 2 + 1) as usize] /= s;
        }
    }
}

fn remove_rotate(a: &mut Vec<f64>, shapes: i32, dots: i32) {
    for i in 1..shapes {
        let mut sum1: f64 = 0.0;
        let mut sum2: f64 = 0.0;
        for j in 0..dots {
            let w = a[(i * dots * 2 + j * 2) as usize];
            let z = a[(i * dots * 2 + j * 2 + 1) as usize];
            let x = a[(j * 2) as usize];
            let y = a[(j * 2 + 1) as usize];

            sum1 += w * y - z * x;
            sum2 += w * x + z * y;
        }
        if sum1 == 0.0 || sum2 == 0.0 {
            continue;
        }
        let t = sum1 / sum2;
        let sin = t / (t * t + 1.0).sqrt();
        let cos = 1.0 / (t * t + 1.0).sqrt();
        for j in 0..dots {
            let w = a[(i * dots * 2 + j * 2) as usize];
            let z = a[(i * dots * 2 + j * 2 + 1) as usize];
            a[(i * dots * 2 + j * 2) as usize] = cos * w - sin * z;
            a[(i * dots * 2 + j * 2 + 1) as usize] = sin * w + cos * z;
        }
    }
}

#[wasm_bindgen]
pub fn calc(arr: &[f64], shapes: i32, dots: i32) -> Vec<f64> {
    let mut a = Vec::from(arr);
    
    remove_translate(&mut a, shapes, dots);
    remove_scale(&mut a, shapes, dots);
    remove_rotate(&mut a, shapes, dots);

    a
}