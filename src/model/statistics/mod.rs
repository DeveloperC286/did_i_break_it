use console::style;

pub struct Statistics {
    skipped: usize,
    failed: usize,
    successful: usize,
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            skipped: 0,
            failed: 0,
            successful: 0,
        }
    }

    pub fn report(&self) {
        println!("{} - {}", style("Skipped").yellow(), self.skipped);
        println!("{} - {}", style("Failed").red(), self.failed);
        println!("{} - {}", style("Successful").green(), self.successful);
        println!("Total - {}", self.skipped + self.failed + self.successful);
    }

    pub fn increment_skipped(&mut self) {
        self.skipped += 1;
    }

    pub fn increment_failed(&mut self) {
        self.failed += 1;
    }

    pub fn increment_successful(&mut self) {
        self.successful += 1;
    }
}
