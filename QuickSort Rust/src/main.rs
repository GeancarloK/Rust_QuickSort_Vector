//mod libscan;
//use libscan::{scan_bool, scan_f32, scan_i32, scan_string};

fn piece_quicksort(vetor: &mut Vec<i32>) {
    if vetor.len() > 1 {
        quicksort(vetor);
    }
}
fn add_quicksort_final(vetor: &mut Vec<i32>, vetor_m: &mut Vec<i32>, posic: &mut i32) {
    for i in 0..vetor_m.len() {
        vetor[*posic as usize] = vetor_m[i];
        *posic += 1;
    }
}

fn quicksort(vetor: &mut Vec<i32>) {
    let post: i32 = vetor[0];
    let mut min: Vec<i32> = Vec::new();
    let mut max: Vec<i32> = Vec::new();
    let mut posic: i32 = 0;
    for i in 1..vetor.len() {
        if vetor[i] < post {
            min.push(vetor[i]);
        } else {
            max.push(vetor[i]);
        }
    }
    piece_quicksort(&mut min);
    piece_quicksort(&mut max);
    add_quicksort_final(vetor, &mut min, &mut posic);
    add_quicksort_final(vetor, &mut vec![post], &mut posic);
    add_quicksort_final(vetor, &mut max, &mut posic);
}

fn main() {
    let mut vetor: &mut Vec<i32> =
        &mut vec![5138, 3751, 9486, 7130, 0178, 2642, 4138, 1890, 8756, 6213];
    quicksort(vetor);
    println!("{:?}", vetor);
}
