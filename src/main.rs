use std::io;
use std::thread;
use std::time::Instant;

fn main() {
    println!("Ensure all other programs are closed before running! If you consistently get a high CV value, try using 1 less thread than the max you have!");
    println!("It is reccomended you run this test with stock settings first!\n");

    let mut input_search_size = String::new();
    println!("\nSearch size (Higher numbers will lead to less irregularities in the data. 10000000 and above seems to give very consistant results. Note that tests are only valid if they share this number): ");
    io::stdin().read_line(&mut input_search_size).unwrap();

    let search_size: u128 = input_search_size.trim().parse::<u128>().unwrap();

    //user input for number of threads
    let mut input_threads = String::new();
    println!("Threads: ");
    io::stdin().read_line(&mut input_threads).unwrap();
    let threads: u128 = input_threads.trim().parse::<u128>().unwrap();

    //user input for number of repeats
    let mut input_repeats = String::new();
    println!("Repeats (Higher numbers will lead to more accurate results as well as more accurately detecting anomalies like throttling, lower number will take less time. 10 and above seems to be very accurate): ");
    io::stdin().read_line(&mut input_repeats).unwrap();
    let repeats: u128 = input_repeats.trim().parse::<u128>().unwrap();

    let mut mean_values: Vec<f64> = vec![];
    let mut median_values: Vec<f64> = vec![];
    let mut standard_deviation_values: Vec<f64> = vec![];
    let mut skewness_values: Vec<f64> = vec![];
    let mut cv_values: Vec<f64> = vec![];
    let mut test_names: Vec<String> = vec![];

    loop {
        if test_names.len() >= 2 {
            for x in 0..test_names.len() {
                println!("\n{}:", test_names[x]);
                println!("Mean: {}ms change from stock: {}% ", mean_values[x], (mean_values[0] - mean_values[x]) / (mean_values[0] + mean_values[x]) / 2.0 );
                println!("Median: {}ms change from stock: {}%", median_values[x], (median_values[0] - median_values[x]) / (median_values[0] + median_values[x]) / 2.0 );
                println!("Standard_deviation: {} change from stock: {}%", standard_deviation_values[x], (standard_deviation_values[0] - standard_deviation_values[x]) / (standard_deviation_values[0] + standard_deviation_values[x]) / 2.0 );
                println!("Skewness: {} change from stock: {}%", skewness_values[x], (skewness_values[0] - skewness_values[x]) / (skewness_values[0] + skewness_values[x]) / 2.0 );
                println!("Coefficent of Variation: {} change from stock: {}%", cv_values[x], (cv_values[0] - cv_values[x]) / (cv_values[0] + cv_values[x]) / 2.0 );
            }
        }
        //initialize varables
        let mut calculations_storage: Vec<f64> = vec![];

        println!("Hit Enter when you have made your changes!");
        let mut wait = String::new();
        let _ = io::stdin().read_line(&mut wait);
        println!("Running...\n");

        //loop over number of repeats
        for _ in 0..repeats {
            //current time
            let now = Instant::now();

            //vector of all threads
            let mut handles = vec![];

            //set initial value to 1 and add 1 to threads to avoid adding 0 to y later on
            for _ in 1..threads + 1 {
                //spawn thread
                handles.push(thread::spawn({
                    move || {
                        let mut y: u128 = 0;

                        while y < search_size {
                            let mut temp: u128 = y;
                            let mut count: u128 = 0;

                            while temp != 1 && temp != 0 {
                                if temp % 2 == 0 {
                                    temp = temp / 2;
                                } else {
                                    temp = 3 * temp + 1;
                                }

                                count = count + 1;
                            }

                            y = y + threads;
                        }
                    }
                }));
            }

            //wait for all threads to finish
            for handle in handles {
                handle.join().unwrap();
            }

            //find elapsed time
            let elapsed: std::time::Duration = now.elapsed();

            //push seconds to execute to a list
            calculations_storage.push(elapsed.as_secs_f64() * 1000.0);
        }

        //find mean
        let sum: f64 = Iterator::sum(calculations_storage.iter());

        let length: f64 = calculations_storage.len() as f64;
        let mean = sum / length;
        mean_values.push(mean);
        println!("Mean:   {} miliseconds", mean);

        //find median
        let middle = length / 2.0;
        let median: f64;
        if length > 2.0 {
            if length % 2.0 == 0.0 {
                median = (calculations_storage[middle as usize - 1]
                    + calculations_storage[middle as usize + 1])
                    / 2.0;
            } else {
                median = calculations_storage[middle as usize];
            }
            median_values.push(mean);
            println!("Median: {} miliseconds", median);

            //find standard deviation
            let mut sum_of_squares: f64 = 0.0;
            for x in 0..calculations_storage.len() {
                sum_of_squares = sum_of_squares + (median - calculations_storage[x]).powf(2.0);
            }

            let standard_deviation = (sum_of_squares / (length - 1.0)).sqrt();
            standard_deviation_values.push(standard_deviation);
            println!("Standard Deviation: {}", standard_deviation);

            //find skewness
            let skewness: f64 = (3.0 * (mean - median)) / standard_deviation;
            skewness_values.push(skewness);
            println!("Skewness: {}", skewness);

            //find CV
            let cv = (standard_deviation / median) * 100.0;
            cv_values.push(cv);
            println!("Coefficent of Variation: {}%", cv);

            //detect a low CV
            if (standard_deviation / mean) * 100.0 > 10.0 {
                println!(
                    "Coefficent of Variation: is high, this may indicate an unstable system or a low number of repeats."
                )
            }

            //detect a degrigation in score
            if skewness > 5.0 {
                println!(
                    "Skewness is high, this may indicate a degradation in score due to throttling."
                )
            }
        } else {
            println!("WARNING: Some data may be unavailable, as you only have {} repeats. Set your repeats > 3 to show this data.", repeats)
        }

        println!("\nName your test: ");

        let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .unwrap();

        test_names.push(name);
    }
}
