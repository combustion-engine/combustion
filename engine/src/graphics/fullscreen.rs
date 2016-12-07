use glfw::{Glfw, Window, Monitor, WindowMode, VidMode, SwapInterval};

pub struct Toggle {
    is_fullscreen: bool,
    last_size: (i32, i32),
    last_pos: (i32, i32)
}

impl Default for Toggle {
    #[inline(always)]
    fn default() -> Toggle { Toggle::new() }
}

impl Toggle {
    pub fn new() -> Toggle {
        Toggle {
            is_fullscreen: false,
            last_pos: (0, 0),
            last_size: (0, 0)
        }
    }

    pub fn toggle(&mut self, glfw: &mut Glfw, window: &mut Window) {
        if self.is_fullscreen {
            window.set_monitor(WindowMode::Windowed, self.last_pos.0, self.last_pos.1, self.last_size.0 as u32, self.last_size.1 as u32, None);
            info!("Window restored to {:?} at location {:?}", self.last_size, self.last_pos);
        } else {
            self.last_pos = window.get_pos();
            self.last_size = window.get_size();

            glfw.with_primary_monitor_mut(|_: &mut _, m: Option<&Monitor>| {
                let monitor = m.unwrap();

                let mode: VidMode = monitor.get_video_mode().unwrap();

                window.set_monitor(WindowMode::FullScreen(&monitor), 0, 0, mode.width, mode.height, Some(mode.refresh_rate));

                info!("{}x{} fullscreen enabled at {}Hz on monitor {}", mode.width, mode.height, mode.refresh_rate, monitor.get_name());
            });
        }

        self.is_fullscreen = !self.is_fullscreen;
    }
}
