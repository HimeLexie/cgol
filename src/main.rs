use std::env;
use rand::Rng;
use crossterm;

fn random_start(x_size: &u64, y_size: &u64) -> Vec<Vec<u8>> {
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

fn pad_array(arr: &Vec<Vec<u8>>) -> Vec<Vec<u8>>{
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

fn print_2d_arr(arr: &Vec<Vec<u8>>) {
    let mut pixel_arr = vec![vec![' '; arr[0].len()]; arr.len()];
    for (i, i_value) in arr.iter().enumerate(){
        for (j, j_value) in i_value.iter().enumerate() {
            let pixel: char = match *j_value {
                0 => ' ',
                _ => '█'
            }; 
            pixel_arr[i][j]=pixel           
        }
    }
    
}

fn cgol_step(in_arr: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut out_arr: Vec<Vec<u8>> = vec![vec![0; in_arr[0].len()]; in_arr.len()];
    for (i, i_value) in in_arr[1..=in_arr.len()-2].iter().enumerate(){
        let mut i = i;
        i += 1;
        for (j, j_value) in i_value[1..=i_value.len()-2].iter().enumerate() {
            let mut j = j;
            j += 1;
            let mut neighbors = 0;
            for k in &in_arr[i-1..=i+1]{
                for l in &k[j-1..=j+1] {
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

fn main() {
    let (mut x, mut y)= match crossterm::terminal::size() {
        Ok(tuple) => match tuple {
            (a, b) => (a as u64, b as u64)
        },
        Err(err) => panic!("{err:?}") 
    };
    let mut args = vec![env::args().collect::<Vec<_>>()[1].parse::<u64>().unwrap()];
    if env::args().len() > 2 {
        args=env::args()
             .collect::<Vec<_>>()[1..=3]
             .iter()
             .map(|x| x.parse::<u64>().unwrap())
             .collect();
        (x, y)=(args[1], args[2])
    } else {
        x -= 2;
        y -= 2;
    }
    let rand_arr: &Vec<Vec<u8>> = &random_start(&x, &y);
    let mut pad_arr = pad_array(rand_arr);
    for _i in 0..args[0] {
        pad_arr=cgol_step(pad_arr);
        print_2d_arr(&pad_arr);
    }
    print!("\x1bc");
}