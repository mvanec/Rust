
/*
 * Sorting function
 */
pub fn bucket_sort(chaos: &mut Vec<i32>, bucket_count: i32) {
    // 1. Create a list of n buckets for an array of lists
    // 2. Put the elements of chaos into buckets
    // 3. Sort each bucket, use insertion sort
    // 4. Create return list, order
    let mut minimum:  i32 = chaos[0];
    let mut maximum:  i32 = chaos[0];
    for value in &mut *chaos {
        if value < &mut minimum {
            minimum = *value;
        }
        if value > &mut maximum {
            maximum = *value;
        }
    }

    let range: i32 = (maximum - minimum + 1) / bucket_count;

    let mut buckets: Vec<Vec<i32>> = Vec::new();
    for _i in 0..bucket_count {
        buckets.push(Vec::new());
    }

    for value in &mut *chaos {
        let mut index: usize = ((*value - minimum) / range).try_into().unwrap();
        if index >= buckets.len() {
            index = buckets.len() - 1;
        }
        let _ = &buckets[index].push(*value);
    }

    for bucket in &mut *buckets {
        bucket.sort();
    }
    let mut index = 0;
    for bucket in &buckets {
        for value in bucket {
            chaos[index] = *value;
            index += 1;
        }
    }
}