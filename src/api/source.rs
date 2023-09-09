use notify::*;
use serde::{Serialize, Deserialize};
use std::{path::{PathBuf, Path}, sync::{Arc, atomic::{AtomicBool, Ordering}}, ops::{DerefMut, Deref}};

#[derive(Serialize, Deserialize)]
pub struct Source {
	path: PathBuf,
	data: String,
	#[serde(skip)]
	watcher: Option<RecommendedWatcher>,
	#[serde(skip)]
	refresh: Arc<AtomicBool>
}

impl Source {
	pub fn new(path: impl AsRef<Path> + Into<PathBuf>) -> Self {
		let update_flag = Arc::new(AtomicBool::new(false));

		Self {
			data: std::fs::read_to_string(path.as_ref()).unwrap_or_default(),
			path: path.into(),
			refresh: update_flag,
			watcher: None
		}
	}

	pub fn assert_watcher_spawned(&mut self) {
		if self.watcher.is_none() {
			let watcher_flag = Arc::clone(&self.refresh);
			
			self.watcher = notify::recommended_watcher(move |res: Result<Event>| {
				if let Ok(res) = res {
					if res.kind.is_modify() {
						watcher_flag.store(true, Ordering::Relaxed);
					}
				}
			}).ok();

			if let Some(watcher) = self.watcher.as_mut() {
				watcher.watch(self.path.as_ref(), notify::RecursiveMode::NonRecursive).ok();
			}
		}
	}

	pub fn wants_reload(&self) -> bool {
		self.refresh.load(Ordering::Relaxed)
	}

	pub fn update_content_from_file(&mut self) {
		if self.refresh.load(Ordering::Relaxed) {
			self.data = std::fs::read_to_string(&self.path).unwrap_or_default();
			self.refresh.store(false, Ordering::Relaxed);
		}
	}

	pub fn get_path(&self) -> &PathBuf {
		&self.path
	}
}

impl AsRef<str> for Source {
    fn as_ref(&self) -> &str {
		&self.data
    }
}

impl Deref for Source {
	type Target = String;

	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl DerefMut for Source {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}