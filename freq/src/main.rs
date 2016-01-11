#[doc="
Counts the frequencies of words read from the standard input, and print
a sorted frequency table.

Assumptions:
"]
fn main() {
    // let word_counts = read_and_count(...);
    // print_counts(word_counts);
}

type CountTable = std::collections::HashMap<String, usize>;

// bool
// char
// usize (like size_t)
// ssize (like ssize_t)
// u8, u16, u32, u64
// s8, ...
// f32, f64
//
// String (like std::string)
// &str (like const char*)

// impl<K, V, S> HashMap<K, V, S> {
//     fn get(&self, k: &K) -> Option<&V> {
//         ...
//     }
// }

// let π = e;
// stms...
//
//   ===
//
// match e {
//   π => stms...
// }

fn increment_word(mut map: &mut CountTable, word: String) {
    *map.entry(word).or_insert(0) += 1;
}

#[cfg(test)]
mod increment_word_tests {
    use super::{increment_word, CountTable};

    #[test]
    fn inserts_if_absent() {
        let mut under_test = fixture();
        let mut expected   = fixture();

        increment_word(&mut under_test, "one".to_owned());
        expected.insert("one".to_owned(), 1);

        assert_eq!(expected, under_test);
    }

    #[test]
    fn increments_if_present() {
        let mut under_test = fixture();
        let mut expected   = fixture();

        increment_word(&mut under_test, "two".to_owned());
        expected.insert("two".to_owned(), 3);

        assert_eq!(expected, under_test);
    }

    #[test]
    fn inserts_if_empty() {
        let mut under_test = CountTable::new();
        let mut expected   = CountTable::new();

        increment_word(&mut under_test, "one".to_owned());
        expected.insert("one".to_owned(), 1);

        assert_eq!(expected, under_test);
    }


    fn fixture() -> CountTable {
        let mut h = CountTable::new();
        h.insert("two".to_owned(), 2);
        h.insert("three".to_owned(), 3);

        assert_eq!(None, h.get("one"));
        assert_eq!(Some(&2), h.get("two"));
        assert_eq!(Some(&3), h.get("three"));
        assert_eq!(2, h.len());

        return h;
    }
}
