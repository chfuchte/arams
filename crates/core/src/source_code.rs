pub trait IntoSourceCode {
    fn into_lines(self) -> Vec<String>;
}

impl IntoSourceCode for Vec<String> {
    fn into_lines(self) -> Vec<String> {
        self
    }
}

impl IntoSourceCode for Vec<&str> {
    fn into_lines(self) -> Vec<String> {
        self.into_iter().map(|line| line.to_string()).collect()
    }
}

impl IntoSourceCode for &str {
    fn into_lines(self) -> Vec<String> {
        self.lines().map(|line| line.to_string()).collect()
    }
}

impl IntoSourceCode for String {
    fn into_lines(self) -> Vec<String> {
        self.lines().map(|line| line.to_string()).collect()
    }
}

impl IntoSourceCode for &String {
    fn into_lines(self) -> Vec<String> {
        self.lines().map(|line| line.to_string()).collect()
    }
}

impl IntoSourceCode for &[String] {
    fn into_lines(self) -> Vec<String> {
        self.iter().cloned().collect()
    }
}

impl IntoSourceCode for &[&str] {
    fn into_lines(self) -> Vec<String> {
        self.iter().map(|&line| line.to_string()).collect()
    }
}
