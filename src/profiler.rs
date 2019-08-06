use std::collections::VecDeque;

pub struct Profiler {
	pub delays: VecDeque<f32>,
	pub view_window: i32,
}

impl Profiler {
	pub fn new(view_window: i32) -> Self {
		Self {
			delays: VecDeque::new(),
			view_window,
		}
	}

	pub fn record_delay(&mut self, delay: f32) {
		while self.delays.len() as i32 > self.view_window {
			self.delays.pop_front();
		}
		self.delays.push_back(delay);
	}

	// Forcefully get the entire delay contents as Vector
	pub fn as_vec(&self) -> Vec<f32> {
		self.delays.iter().copied().collect()
	}

	pub fn draw_ui(&mut self, delta_time: f32, ui: &mut imgui::Ui) {
		use imgui::Condition;
		imgui::Window::new(ui, im_str!("Profiler"))
			.size([400.0, 125.0], Condition::FirstUseEver)
			.position([50.0, 200.0], Condition::FirstUseEver)
			.build(|| {
				ui.text(format!("FPS: {:.2}/{:.5}ms", 1.0 / delta_time, delta_time));
				ui.slider_int(im_str!("View window"), &mut self.view_window, 10, 1000)
					.build();

				ui.plot_lines(im_str!("Delay (ms)"), self.as_vec().as_ref())
					.graph_size([300.0, 75.0])
					.build();
			});
	}
}
