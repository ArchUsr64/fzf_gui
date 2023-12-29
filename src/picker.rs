use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub struct Search {
	query: String,
	cursor: usize,
}

impl Search {
	fn new() -> Self {
		Self {
			query: "".into(),
			cursor: 0,
		}
	}
	pub fn insert(&mut self, ch: char) {
		let (left, right) = self.query.split_at(self.cursor);
		let mut new_query = String::from(left);
		new_query.push(ch);
		new_query.push_str(right);
		self.query = new_query;
		self.cursor += 1;
	}
	pub fn cursor_to_start(&mut self) {
		self.cursor = 0;
	}
	pub fn cursor_to_end(&mut self) {
		self.cursor = self.query.len();
	}
	pub fn delete_word(&mut self) {
		loop {
			if self.cursor == 0 {
				return;
			}
			let ch = self.query.remove(self.cursor - 1);
			self.cursor -= 1;
			if ch == ' ' {
				break;
			}
		}
		while let Some(' ') = self.query.chars().nth(self.cursor - 1) {
			self.query.remove(self.cursor - 1);
			self.cursor -= 1;
		}
	}
	pub fn delete_till_start(&mut self) {
		let (_, right) = self.query.split_at(self.cursor);
		self.query = right.to_string();
		self.cursor = 0;
	}
	pub fn delete_till_end(&mut self) {
		let (left, _) = self.query.split_at(self.cursor);
		self.query = left.to_string();
	}
	pub fn delete(&mut self) {
		if self.cursor > 0 {
			self.cursor -= 1;
			let (left, right) = self.query.split_at(self.cursor);
			self.query = format!("{left}{}", &right[1..]);
		}
	}
	pub fn cursor_left(&mut self) {
		self.cursor = self.cursor.saturating_sub(1);
	}
	pub fn cursor_right(&mut self) {
		self.cursor += 1;
		if self.cursor > self.query.len() {
			self.cursor = self.query.len();
		}
	}
}

pub struct Picker {
	pub search: Search,
	matches: Vec<(i64, usize)>,
	options: Vec<String>,
}

impl Picker {
	pub fn new(options: Vec<String>) -> Self {
		Self {
			search: Search::new(),
			matches: Vec::new(),
			options,
		}
	}
	pub fn query(&self) -> &str {
		&self.search.query
	}
	pub fn cursor(&self) -> usize {
		self.search.cursor
	}
	pub fn update(&mut self) {
		self.matches.clear();
		let fuzzy_matcher = SkimMatcherV2::default();
		for (i, choice) in self.options.iter().enumerate() {
			if let Some(score) = fuzzy_matcher.fuzzy_match(choice, self.query()) {
				self.matches.push((score, i));
			};
		}
		self.matches.sort_by(|a, b| b.0.cmp(&a.0));
	}
	pub fn get_matches(&self, count: usize) -> impl Iterator<Item = &str> {
		self.matches[..count.min(self.matches.len())]
			.iter()
			.map(|(_, i)| self.options[*i].as_str())
	}
}
