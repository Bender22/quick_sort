mod sorts;
mod methods;
mod trait_to_vec;

use rand::prelude::SliceRandom;
use rand::thread_rng;
use crate::sorts::merge_sort::merge_sort;
use crate::sorts::quick_sort::quick_sort;
use crate::sorts::tim_sort::{tim_sort, tim_sort_quick};
use crate::trait_to_vec::traits::Sort;

impl<T: Clone + Ord> Sort for Vec<T>{
    fn quick_sort(&mut self) {
        quick_sort::<T>(self);
    }

    fn merge_sort(&mut self) {
        merge_sort::<T>(self)
    }

    fn tim_sort(&mut self) {
        tim_sort(self);
    }

    fn tim_sort_quick(&mut self) {
        tim_sort_quick(self)
    }
}

fn fill_array_with_random_numbers(size: i32) ->  Vec<i32> {
    let mut arr:Vec<i32>  = (1..=size).collect();
    // for i in 0..1000000 {
    //     arr[i] = (i + 1) as i32;
    // }

    // Mezclar el array para obtener un orden aleatorio
    let mut rng = thread_rng();
    arr.shuffle(&mut rng);
    // let mut rng = rand::thread_rng();
    // for i in arr.iter_mut() {
    //     *i = rng.gen_range(1..88888)
    // }
    arr

}

fn main() {
    let mut sort_array =  fill_array_with_random_numbers(1000000);
    // let mut sort_array =  vec![1; 1000000];
    let _len = sort_array.len();
    println!("start sorting with length {}", sort_array.len());
    let start_time = std::time::Instant::now();
    // merge_sort(&mut sort_array);
    // tim_sort_a(&mut sort_array);
    // quick_sort::<i32>(&mut sort_array);

    // sort_array.quick_sort();
    // sort_array.merge_sort();
    sort_array.tim_sort();
    // sort_array.tim_sort_quick();
    println!("Sort in  {:?} {:?}", start_time.elapsed(), &sort_array[0..15]);
}


use std::cmp;
use crate::methods::methods::merge;

// fn insertion_sort<T: Ord>(arr: &mut [T]) {
//     for i in 1..arr.len() {
//         let mut j = i;
//         while j > 0 && arr[j] < arr[j - 1] {
//             arr.swap(j, j - 1);
//             j -= 1;
//         }
//     }
// }

fn median_of_three<T: Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
    let mid = (low + high) / 2;
    if arr[low] > arr[mid] {
        arr.swap(low, mid);
    }
    if arr[low] > arr[high] {
        arr.swap(low, high);
    }
    if arr[mid] > arr[high] {
        arr.swap(mid, high);
    }
    mid
}

fn partition<T: Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot_index = median_of_three(arr, low, high);
    arr.swap(pivot_index, high);

    let mut i = low;
    for j in low..high {
        if arr[j] < arr[high] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

fn quick_sort_a<T: Ord>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot_index = partition(arr, low, high);
        if pivot_index > 0 {
            quick_sort_a(arr, low, pivot_index - 1);
        }
        quick_sort_a(arr, pivot_index + 1, high);
    }
}

// fn merge<T: Ord + Clone>(left: &[T], right: &[T], dest: &mut [T]) {
//     let mut i = 0;
//     let mut j = 0;
//     let mut k = 0;
// 
//     while i < left.len() && j < right.len() {
//         if left[i] <= right[j] {
//             dest[k] = left[i].clone();
//             i += 1;
//         } else {
//             dest[k] = right[j].clone();
//             j += 1;
//         }
//         k += 1;
//     }
// 
//     if i < left.len() {
//         dest[k..].clone_from_slice(&left[i..]);
//     }
// 
//     if j < right.len() {
//         dest[k..].clone_from_slice(&right[j..]);
//     }
// }

fn tim_sort_a<T: Ord + Clone>(arr: &mut [T]) {
    let min_run = 32;

    // Sort small runs with quick sort
    for i in (0..arr.len()).step_by(min_run) {
        let end = cmp::min(i + min_run, arr.len());
        quick_sort_a(&mut arr[i..end], 0, end - i - 1);
    }

    // Merge sorted runs
    let mut size = min_run;
    while size < arr.len() {
        for start in (0..arr.len()).step_by(size * 2) {
            let mid = start + size;
            let end = cmp::min(start + size * 2, arr.len());

            if mid < end {
                let mut merged = vec![arr[start].clone(); end - start];
                merge(&mut merged, &arr[start..mid], &arr[mid..end]);
                arr[start..end].clone_from_slice(&merged);
            }
        }
        size *= 2;
    }
}


