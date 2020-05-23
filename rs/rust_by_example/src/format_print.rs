use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (count, value) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}:{}", count, value)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format_print() {
        println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");

        println!("{} of {:b} people know binary", 1, 10);
    }

    #[test]
    fn impl_display() {
        let v = List(vec![1, 2, 3]);
        println!("{}", v)
    }
}