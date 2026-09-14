pub struct Progress {
	total: usize,
	completed: usize,
	summaries: Vec<String>,
}

impl Progress {
	pub fn new(total: usize) -> Self {
		Self {
			total,
			completed: 0,
			summaries: Vec::new(),
		}
	}

	pub fn tick(&mut self, summary: &str) {
		self.completed += 1;
		self.summaries.push(summary.to_string());

		eprintln!("[{}/{}] {summary}", self.completed, self.total);
	}

	pub fn print_summary(&self) {
		eprintln!("\n=== Seeding complete ===");

		for s in &self.summaries {
			eprintln!("  {s}");
		}
	}
}
