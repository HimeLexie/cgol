use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;

fn random_start(x_size: &u64, y_size: &u64) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();
    let mut col: Vec<Vec<bool>> = Vec::new();
    for _i in 1..=*y_size {
        let mut row = Vec::new();
        for _j in 1..=*x_size {
            // rand::rngs::adapter::ReseedingRng::reseed(&mut self);
            row.push(rng.gen_bool(0.5)); 
        }
        col.push(row);
    };
    col
}

fn pad_array(arr: &Vec<Vec<bool>>) -> Vec<Vec<bool>>{
    let mut pad_arr: Vec<Vec<bool>> = Vec::new();
    for i in 0..=(arr.len() + 1) {
        if i == 0 || i == arr.len() + 1 {
            pad_arr.push(vec![false; arr[0].len() + 2])
        } else {
            let mut pad_row = vec![false];
            pad_row.extend(&arr[i - 1]);
            pad_row.push(false);
            pad_arr.push(pad_row)
        }      
    };
    pad_arr
}

fn cgol_step(in_arr: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut out_arr: Vec<Vec<bool>> = vec![vec![false; in_arr[0].len()]; in_arr.len()];
    for (i, i_value) in in_arr[1..=in_arr.len()-2].iter().enumerate(){
        let mut i = i;
        i += 1;
        for (j, j_value) in i_value[1..=i_value.len()-2].iter().enumerate() {
            let mut j = j;
            j += 1;
            let neighbor_arr: Vec<&[bool]> = in_arr[i-1..=i+1].iter()
            .map(|s| &s[j-1..=j+1]).collect();
            let mut neighbors = 0;
            for k in neighbor_arr {
                for l in k {
                    neighbors += *l as u8;
                }
            }
            let j_value = *j_value;
            neighbors -= j_value as u8;
            if j_value == true && neighbors < 2 {
                out_arr[i][j]=false
            } else if j_value == true && neighbors >= 2 && neighbors <= 3 {
                out_arr[i][j]=true
            } else if j_value == true && neighbors > 3 {
                out_arr[i][j]=false
            } else if j_value == false && neighbors == 3 {
                out_arr[i][j]=true
            }
        }
    };
    out_arr
}

fn criterion_benchmark(c: &mut Criterion) {
    let pad_arr = pad_array(&random_start(&1000, &1000));

    c.bench_with_input(BenchmarkId::new("int_cgol", "1000x1000 random"), &pad_arr,
        |b, pad_arr| b.iter(|| {
            // Code to benchmark goes here
            cgol_step(&pad_arr);
        })
    );
}

fn random_start_int(x_size: &u64, y_size: &u64) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut col: Vec<Vec<u8>> = Vec::new();
    for _i in 1..=*y_size {
        let mut row = Vec::new();
        for _j in 1..=*x_size {
            // rand::rngs::adapter::ReseedingRng::reseed(&mut self);
            row.push(rng.gen_range(0..=1)); 
        }
        col.push(row);
    };
    col
}

fn pad_array_int(arr: &Vec<Vec<u8>>) -> Vec<Vec<u8>>{
    let mut pad_arr = Vec::new();
    for i in 0..=(arr.len() + 1) {
        if i == 0 || i == arr.len() + 1 {
            pad_arr.push(vec![0; arr[0].len() + 2])
        } else {
            let mut pad_row = vec![0];
            pad_row.extend(&arr[i - 1]);
            pad_row.push(0);
            pad_arr.push(pad_row)
        }      
    };
    pad_arr
}

fn cgol_step_int(in_arr: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut out_arr: Vec<Vec<u8>> = vec![vec![0; in_arr[0].len()]; in_arr.len()];
    for (i, i_value) in in_arr[1..=in_arr.len()-2].iter().enumerate(){
        let mut i = i;
        i += 1;
        for (j, j_value) in i_value[1..=i_value.len()-2].iter().enumerate() {
            let mut j = j;
            j += 1;
            let neighbor_arr: Vec<&[u8]> = in_arr[i-1..=i+1].iter()
            .map(|s| &s[j-1..=j+1]).collect();
            let mut neighbors = 0;
            for k in neighbor_arr {
                for l in k {
                    neighbors += *l;
                }
            }
            let j_value = *j_value;
            neighbors -= j_value;
            if j_value == 1 && neighbors < 2 {
                out_arr[i][j]=0
            } else if j_value == 1 && neighbors >= 2 && neighbors <= 3 {
                out_arr[i][j]=1
            } else if j_value == 1 && neighbors > 3 {
                out_arr[i][j]=0
            } else if j_value == 0 && neighbors == 3 {
                out_arr[i][j]=1
            }
        }
    };
    out_arr
}

fn int_benchmark(c: &mut Criterion) {
    let pad_arr = pad_array_int(&random_start_int(&1000, &1000));

    c.bench_with_input(BenchmarkId::new("int_cgol", "1000x1000 random"), &pad_arr,
|b, pad_arr| b.iter(|| {
            // Code to benchmark goes here
            cgol_step_int(&pad_arr);
        })
    );
}

criterion_group!(benches, criterion_benchmark, int_benchmark);
criterion_main!(benches);