#![allow(unused_variables)]

pub fn sum_cached() {
    let mut v = vec![0; 1600000000]; // create a vec
    // populating the vector
    v.iter_mut().enumerate().for_each(|(idx, elem)| {
        *elem = idx as u64 + 1;
    });
    let mut sum = 0u64;
    for x in 0..40000 {
        for y in 0..40000 {
            sum += v[x * 40000 + y];
        }
    }
    println!("Sum is {}", sum);
}
pub fn sum_uncached() {
    let mut v = vec![0; 1600000000];
    v.iter_mut().enumerate().for_each(|(idx, elem)| {
        *elem = idx as u64 + 1;
    });
    let mut sum = 0u64;
    for x in 0..40000 {
        for y in 0..40000 {
            sum += v[x + 40000 * y];
        }
    }
    println!("Sum is {}", sum);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_sum_cached() {
        println!("\nCompute cached");
        let now = Instant::now();
        sum_cached();
        println!("Done in {:.2?}\n", now.elapsed());
    }
    #[test]
    fn test_sum_uncached() {
        println!("\nCompute uncached");
        let now = Instant::now();
        sum_uncached();
        println!("Done in {:.2?}\n", now.elapsed());
    }
}
