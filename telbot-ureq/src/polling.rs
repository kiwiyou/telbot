use telbot_types::update::{GetUpdates, Update};

use crate::{Api, Result};

pub struct Polling<'a> {
    api: &'a Api,
    offset: u32,
    timeout: u32,
    queue: Vec<Update>,
}

impl<'a> Polling<'a> {
    /// Create a new Polling object with default timeout 1s.
    pub fn new(api: &'a Api) -> Self {
        const DEFAULT_TIMEOUT: u32 = 1;

        Self {
            api,
            offset: 0,
            timeout: DEFAULT_TIMEOUT,
            queue: vec![],
        }
    }
}

impl Iterator for Polling<'_> {
    type Item = Result<Update>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.queue.is_empty() {
            let updates = self.api.send_json(
                &GetUpdates::new()
                    .with_offset(self.offset as i32)
                    .with_timeout(self.timeout),
            );
            match updates {
                Ok(update) => {
                    self.queue = update;
                    self.offset = self
                        .queue
                        .iter()
                        .map(|update| update.update_id + 1)
                        .fold(self.offset, std::cmp::max);
                }
                Err(e) => return Some(Result::Err(e)),
            }
        }
        self.queue.pop().map(Result::Ok)
    }
}
