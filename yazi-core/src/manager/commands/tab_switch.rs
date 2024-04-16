use yazi_shared::{event::Cmd, render};

use crate::{manager::Tabs, tab::Tab};

pub struct Opt {
	step:     isize,
	relative: bool,
	create: bool,
}

impl From<Cmd> for Opt {
	fn from(mut c: Cmd) -> Self {
		Self {
			step:     c.take_first().and_then(|s| s.parse().ok()).unwrap_or(0),
			relative: c.named.contains_key("relative"),
			create:   c.named.contains_key("create"),
		}
	}
}

impl Tabs {
	pub fn switch(&mut self, opt: impl Into<Opt>) {
		let opt = opt.into() as Opt;
		let idx = if opt.relative {
			(self.cursor as isize + opt.step).rem_euclid(self.items.len() as isize) as usize
		} else {
			opt.step as usize
		};

		if idx == self.cursor {
			return;
		}

        while idx >= self.items.len() {
            let mut tab = Tab::default();
			tab.conf = self.active().conf.clone();
			tab.apply_files_attrs();
			tab.cd(self.active().current.cwd.clone());

            self.items.insert(self.cursor + 1, tab);
            self.set_idx(self.cursor + 1);
        }

		self.set_idx(idx);
        self.reorder();
		render!();
	}
}
