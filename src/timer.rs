use std::time::Duration;

#[derive(Debug, Default)]
pub struct Timer {
    current_state: TimerState,
    start_time: Option<std::time::Instant>,
    target_duration: Duration,
}

#[derive(Copy, Clone, Debug, Default)]
enum TimerState {
    #[default]
    Idle,
    Running,
    Paused,
    Completed,
}

#[derive(Copy, Clone)]
pub enum TimerEvent {
    StartWork,
    StartBreak,
    Pause,
    Resume,
    Reset,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            current_state: TimerState::Idle,
            start_time: None,
            target_duration: Duration::ZERO,
        }
    }

    pub fn handle_input(&mut self, input: TimerEvent) {
        match (self.current_state, input) {
            (TimerState::Idle, TimerEvent::StartWork) => {
                self.start_work();
            }
            (TimerState::Idle, TimerEvent::StartBreak) => {
                self.start_break();
            }
            (TimerState::Running, TimerEvent::Pause) => {
                self.pause();
            }
            (TimerState::Paused, TimerEvent::Resume) => {
                self.handle_resume();
            }
            (TimerState::Paused, TimerEvent::Reset) => {
                self.reset();
            }
            (TimerState::Completed, TimerEvent::Reset) => {
                self.reset();
            }
            (_, _) => {}
        }
    }
    pub fn get_remaining_time(&mut self) -> Duration {
        match self.current_state {
            TimerState::Running => {
                let elapsed = self.start_time.unwrap().elapsed();
                let remaining = self.target_duration.saturating_sub(elapsed);
                if remaining == Duration::ZERO {
                    self.current_state = TimerState::Completed;
                }
                remaining
            }
            TimerState::Paused => self.target_duration,
            TimerState::Completed => Duration::ZERO,
            TimerState::Idle => Duration::ZERO,
        }
    }

    fn reset(&mut self) {
        self.current_state = TimerState::Idle;
        self.target_duration = Duration::from_secs(25 * 60);
    }

    fn start_work(&mut self) {
        self.current_state = TimerState::Running;
        self.start_time = Some(std::time::Instant::now());
        self.target_duration = Duration::from_secs(25 * 60);
    }

    fn start_break(&mut self) {
        self.current_state = TimerState::Running;
        self.start_time = Some(std::time::Instant::now());
        self.target_duration = Duration::from_secs(5 * 60);
    }

    fn pause(&mut self) {
        self.current_state = TimerState::Paused;
        self.target_duration = self
            .target_duration
            .saturating_sub(self.start_time.unwrap().elapsed());
        self.start_time = None;
    }

    fn handle_resume(&mut self) {
        self.current_state = TimerState::Running;
        self.start_time = Some(std::time::Instant::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import Timer, TimerState, TimerEvent from parent module
    use std::thread::{self};
    use std::time::Duration;

    #[test]
    fn test_timer_starts_work_with_25_minutes() {
        // Arrange
        let mut timer = Timer::new(); // You'll need to add a new() method!

        // Act
        timer.handle_input(TimerEvent::StartWork);

        // Assert
        let remaining = timer.get_remaining_time();
        assert!(remaining > Duration::from_secs(24 * 60 + 59)); // At least 24:59
        assert!(remaining <= Duration::from_secs(25 * 60)); // At most 25:00
    }

    #[test]
    fn test_timer_starts_break_with_5_minutes() {
        let mut timer = Timer::new();

        timer.handle_input(TimerEvent::StartBreak);

        let remaining = timer.get_remaining_time();
        assert!(remaining > Duration::from_secs(4 * 60 + 59));
        assert!(remaining <= Duration::from_secs(5 * 60));
    }

    #[test]
    fn test_timer_counts_down() {
        let mut timer = Timer::new();
        timer.handle_input(TimerEvent::StartWork);

        let initial = timer.get_remaining_time();
        thread::sleep(Duration::from_millis(100)); // Wait a bit
        let after = timer.get_remaining_time();

        assert!(after < initial, "Time should decrease");
    }

    #[test]
    fn test_timer_pauses_after_pausing() {
        let mut timer = Timer::new();
        timer.handle_input(TimerEvent::StartWork);
        timer.handle_input(TimerEvent::Pause);
        assert!(matches!(timer.current_state, TimerState::Paused));
    }

    #[test]
    fn test_timer_resumes_after_resuming() {
        let mut timer = Timer::new();
        timer.handle_input(TimerEvent::StartWork);
        timer.handle_input(TimerEvent::Pause);
        let initial_start_time = timer.start_time;

        thread::sleep(Duration::from_millis(100)); // Wait a bit
        timer.handle_input(TimerEvent::Resume);
        assert!(matches!(timer.current_state, TimerState::Running));
        assert!(timer.start_time > initial_start_time);
    }

    #[test]
    fn pause_stops_time() {
        let mut timer = Timer::new();
        timer.handle_input(TimerEvent::StartWork);
        thread::sleep(Duration::from_millis(50)); // Wait a bit

        timer.handle_input(TimerEvent::Pause);
        let remaining_at_pause = timer.get_remaining_time();

        thread::sleep(Duration::from_millis(50)); // Wait a bit more
        let time_after_pause = timer.get_remaining_time();
        assert!(time_after_pause == remaining_at_pause);
    }
}
