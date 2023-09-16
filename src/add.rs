pub struct Add {
    pub filename: String,
}

impl Add {
    fn usage(&self) -> String {
        format!("add {}", self.filename)
    }

    fn run(&self) {
        println!("add {}", self.filename);
    }
}

// add test

#[cfg(test)]
mod test {
    #[test]
    fn test_usage() {
        assert!(true)
    }
}
