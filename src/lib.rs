use wasm_bindgen::prelude::*;
use rand::seq::SliceRandom;
use js_sys::Array;

/**
    Takes an array with elements of any type and shuffles it using shuffle().
 */

#[wasm_bindgen]
pub fn draw(original: Array) -> Array {
    let shuffled_indices = shuffle(original.length() as usize);
    let shuffled = Array::new();

    for i in 0..original.length(){
        shuffled.set(i, original.at(shuffled_indices[i as usize] as i32));
    }
    return shuffled;
}

/**
    Takes the length of a range, each digit of which is randomly and uniquely stored in an array.
    Each digit changes its position.
 */

#[wasm_bindgen]
pub fn shuffle(length: usize) -> Vec<usize> {
    let givers: Vec<usize> = (0..length).collect();
    let mut receivers: Vec<usize> = (0..length).collect();

    let mut rng = rand::thread_rng();
    let mut result: Vec<usize> = Vec::new();

    for giver in givers {
        loop {
            let receiver = receivers.choose(&mut rng).unwrap().clone();
            if receiver != length && giver != receiver {
                result.push(receiver);
                receivers[receiver] = length;
                break;
            }
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    #[test]
    fn result_has_param_length() {
        let result = super::shuffle(10);
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn result_has_no_duplicates() {
        let result = super::shuffle(10);
        let mut set = std::collections::HashSet::new();
        for i in result {
            assert!(set.insert(i));
        }
    }

    #[test]
    fn result_has_no_value_equal_to_index() {
        let result = super::shuffle(10);
        for (i, v) in result.iter().enumerate() {
            assert_ne!(i, *v);
        }
    }
}