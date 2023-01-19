use std::io;
use rand::prelude::*;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc;
// use std::num::Float;

fn main() {
    // println!("Guess the number!");


    println!("Please input the number of πs to generate.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut n: i32 = input.trim().parse()
        .expect("please give me correct string number!");

    let now = Instant::now();

    let mut pis = Vec::new();

    let points_tot = 1000;

    let mut pi_index = 1;

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    //let cycles = 10.0 / 
    let threads = 10;
    let cycles_per_thread = n / threads;

    let mut current_pi_count = 0;

    while pi_index <= threads {
        /*
        let mut i = 1;
        let mut points_in: f64 = 0.0;
        while i <= points_tot {
            let x: f64 = rng.gen();
            let y: f64 = rng.gen();

            if x * x + y * y <= 1.0 {
                points_in += 1.0;
            }

            // println!("x: {x}, y: {y}");
            i += 1;
        }

        // println!("in: {points_in}, total: {n}");

        let pi: f64 = points_in / f64::from(points_tot) * 4.0;
         */
        let tx_temp = tx.clone();
        let handle = thread::spawn(move || {
            /*
            let pi = generate_pi(points_tot);
            tx_temp.send(pi).unwrap();
             */
            //generate_pis(points_tot, cycles_per_thread, tx);
            let mut pi_index_loc = 1;

            while pi_index_loc <= cycles_per_thread {
                let pi = generate_pi(points_tot);
                tx_temp.send(pi).unwrap();
                pi_index_loc += 1;
                current_pi_count += 1;
                let progress = f64::from(current_pi_count) / f64::from(n) * 1000.0;
                println!("{progress}%")
            }

            // let mut progress = pis.len() as f64 / f64::from(n) * 100.0; 
            println!("Finished thread {pi_index} of {threads}");
        });

        handles.push(handle);
        // pis.push(pi);
        pi_index += 1;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for received in rx {
        // println!("{received}");
        pis.push(received);

        if (pis.len() as i32 == n) {
            println!("starting to process...");

            let length = pis.len() as i32;
            let pi = mean(&pis[..]).unwrap();
            let stdev = std_deviation(&pis[..]).unwrap();
            let se = stdev / f64::from(length).sqrt();

            let t = now.elapsed().as_secs();
            println!("Mean: {pi}, SE: {se}, STDEV: {stdev}, n: {length}, points: {points_tot}, t: {t} s");
        }
    }

    println!("starting to process...");

    let length = pis.len() as i32;
    let pi = mean(&pis[..]).unwrap();
    let stdev = std_deviation(&pis[..]).unwrap();
    let se = stdev / f64::from(length).sqrt();

    let t = now.elapsed().as_secs();
    println!("Mean: {pi}, SE: {se}, STDEV: {stdev}, n: {length}, points: {points_tot}, t: {t} s");
}
/*
fn generate_pis(points_tot: i32, n: i32, tx: Sender<f64>) {
    let mut pi_index_loc = 1;

    while pi_index_loc <= n {
        let pi = generate_pi(points_tot);
        let tx_temp = tx.clone();
        tx_temp.send(pi).unwrap();
    }
    
}
 */

fn generate_pi(points_tot: i32) -> f64 {
    let mut rng = rand::thread_rng();

    let mut i = 1;
        let mut points_in: f64 = 0.0;
        while i <= points_tot {
            let x: f64 = rng.gen();
            let y: f64 = rng.gen();

            if x * x + y * y <= 1.0 {
                points_in += 1.0;
            }

            // println!("x: {x}, y: {y}");
            i += 1;
        }

        // println!("in: {points_in}, total: {n}");

        let pi: f64 = points_in / f64::from(points_tot) * 4.0;
        // println!("{pi}");
        return pi;
}

fn mean(data: &[f64]) -> Option<f64> {
    let sum = data.iter().sum::<f64>() as f64;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn std_deviation(data: &[f64]) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f64);

                diff * diff
            }).sum::<f64>() / count as f64;

            Some(variance.sqrt())
        },
        _ => None
    }
}