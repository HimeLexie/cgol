use rand::Rng;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn random_start(x_size: &u16, y_size: &u16) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();
    let mut col = Vec::new();
    for _i in 1..=*y_size {
        let mut row = Vec::new();
        for _j in 1..=*x_size {
            // rand::rngs::adapter::ReseedingRng::reseed(&mut self);
            row.push(rng.gen_bool(0.5)); 
        }
        col.push(row);
    };
    return col;
}

fn pad_array(arr: &Vec<Vec<bool>>) -> Vec<Vec<bool>>{
    let mut pad_arr = Vec::new();
    for i in 0..=(arr.len() + 1) {
        if i == 0 || i == arr.len() + 1 {
            pad_arr.push(vec![false; arr.len() + 2])
        } else {
            let mut pad_row = vec![false];
            pad_row.extend(&arr[i - 1]);
            pad_row.push(false);
            pad_arr.push(pad_row)
        }      
    };
    pad_arr
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("pad_arr 1000x1000",
        |b| b.iter(|| {
            // Code to benchmark goes here
            pad_array(black_box(&random_start(&1000, &1000)))
        })
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);