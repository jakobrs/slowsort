pub fn slowsort<T: Ord>(data: &mut [T]) {
    let m @ 1.. = data.len() / 2 else { return };

    slowsort(&mut data[..m]);
    slowsort(&mut data[m..]);

    if data[m] < data[0] {
        data.swap(0, m);
    }

    slowsort(&mut data[1..]);
}

#[cfg(test)]
mod test {
    use super::*;

    use std::fmt::Debug;

    fn verify<T: Ord + Clone + Debug>(slice: &[T]) {
        let mut correct = slice.to_owned();
        correct.sort();

        let mut result = slice.to_owned();
        slowsort(&mut result);

        assert_eq!(correct, result);
    }

    #[test]
    fn it_works_for_some_fixed_examples() {
        verify(&[1, 2, 3, 4, 5, 4, 3, 2, 5, 8]);
        verify(&[5, 4, 3, 2, 1, 1, 2, 1, 3, 2]);
        verify(&[1, 3, 2, 4, 5, 4, 2, 5, 3, 7]);
        verify(&[1, 2, 3, 2, 4, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn it_works_for_a_bunch_of_randomly_generated_examples() {
        for a in 0..10 {
            for b in 0..10 {
                for c in 0..10 {
                    for d in 0..10 {
                        for e in 0..10 {
                            verify(&[a, b, c, d, e]);
                        }
                    }
                }
            }
        }
    }
}
