struct Parse {
	pos: usize,
	input: String,

}

impl Parse {
	fn next-char(&self) -> char {
		self.input[self.pos..].chars().next().unwrap()
	}

	fn starts_with(&self, s: &str) => bool {
		self.input[self.pos ..].starts_with(s)
	}

	fn eof(&self) -> bool {
		self.pos >= self.input.len()
	}
}

