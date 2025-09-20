// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let word = optional_target {
            let word = word.unwrap();
            // This assertion is to ensure that the value inside the Option is
            // what we expect it to be.
            // If your code is stuck in an infinite loop, this assertion will
            // fail.
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let integer = optional_integers.pop() {
            let integer = integer.unwrap();
            if integer.is_none() {
                break;
            }
            let integer = integer.unwrap();
            // This assertion is to ensure that the integers are being popped in
            // the right order.
            // If your code is stuck in an infinite loop, this assertion will
            // fail.
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
