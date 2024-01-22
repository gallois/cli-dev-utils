use crate::dev_utils::list::*;

mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        assert_eq!(
            sort("one,two,three,four,five", ","),
            "five,four,one,three,two"
        );

        assert_eq!(
            sort("one two three four five", " "),
            "five four one three two"
        );
    }

    #[test]
    fn test_lowercase() {
        assert_eq!(
            lowercase("One Two Three Four FIVE", " "),
            "one two three four five"
        );
        assert_eq!(
            lowercase("One,Two,Three,Four,FIVE", ","),
            "one,two,three,four,five"
        );
    }

    #[test]
    fn test_uppercase() {
        assert_eq!(
            uppercase("One Two Three Four FIVE", " "),
            "ONE TWO THREE FOUR FIVE"
        );
        assert_eq!(
            uppercase("One,Two,Three,Four,FIVE", ","),
            "ONE,TWO,THREE,FOUR,FIVE"
        );
    }

    #[test]
    fn test_capitalise() {
        assert_eq!(
            capitalise("One Two Three Four FIVE", " "),
            "One Two Three Four Five"
        );
        assert_eq!(
            capitalise("One,Two,Three,Four,FIVE", ","),
            "One,Two,Three,Four,Five"
        );
    }

    #[test]
    fn test_reverse() {
        assert_eq!(
            reverse("One Two Three Four Five", " "),
            "Five Four Three Two One"
        )
    }

    #[test]
    fn test_deduplicate() {
        assert_eq!(
            deduplicate("One Two Three Four Five Five", " "),
            "One Two Three Four Five"
        );
        assert_eq!(
            deduplicate("One Two Three Four Five Five Four", " "),
            "One Two Three Four Five"
        );
    }

    #[test]
    fn test_slice() {
        assert_eq!(
            slice("One Two Three Four Five Five Four", " ", 0, 3),
            "One Two Three"
        );
        assert_eq!(
            slice("One Two Three Four Five Five Four", " ", 1, 3),
            "Two Three Four"
        );
        assert_eq!(slice("One Two Three Four Five Five Four", " ", 1, 1), "Two");
        assert_eq!(slice("One Two Three Four Five Five Four", " ", 8, 1), "");
    }

    #[test]
    fn test_count() {
        assert_eq!(count("one,two,three,four,five", ","), 5);
        assert_eq!(count("", " "), 0);
    }
}
