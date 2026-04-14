use pomodog_lib::domain::{App, AppState, InputError, Phase, SessionConfig, TaskName, Timer};

#[test]
fn test_task_name_add_char() {
    let mut name = TaskName::default();
    name.add_char('H');
    name.add_char('e');
    name.add_char('l');
    name.add_char('l');
    name.add_char('o');
    assert_eq!(name.as_str(), "Hello");
}

#[test]
fn test_task_name_length_limit() {
    let mut name = TaskName::default();
    for _ in 0..30 {
        name.add_char('a');
    }
    assert_eq!(name.len(), 25);
}

#[test]
fn test_task_name_remove_char() {
    let mut name = TaskName::default();
    name.add_char('H');
    name.add_char('i');
    name.remove_char();
    assert_eq!(name.as_str(), "H");
}

#[test]
fn test_task_name_remove_word() {
    let mut name = TaskName::default();
    name.add_char('H');
    name.add_char('e');
    name.add_char('l');
    name.add_char('l');
    name.add_char('o');
    name.add_char(' ');
    name.add_char('w');
    name.add_char('o');
    name.add_char('r');
    name.add_char('l');
    name.add_char('d');
    name.remove_word();
    assert_eq!(name.as_str(), "Hello ");
}

#[test]
fn test_task_name_clear() {
    let mut name = TaskName::default();
    name.add_char('T');
    name.add_char('e');
    name.add_char('s');
    name.add_char('t');
    name.clear();
    assert!(name.is_empty());
}

#[test]
fn test_task_name_invalid_chars_rejected() {
    let mut name = TaskName::default();
    name.add_char('\u{200B}');
    name.add_char('a');
    assert_eq!(name.as_str(), "a");
}

#[test]
fn test_timer_tick() {
    let mut timer = Timer::default();
    timer.reset(5);
    assert_eq!(timer.formatted_time(), "00:05");
    timer.tick();
    assert_eq!(timer.formatted_time(), "00:04");
}

#[test]
fn test_timer_progress_ratio() {
    let mut timer = Timer::default();
    timer.reset(100);
    timer.tick();
    timer.tick();
    timer.tick();
    assert!((timer.progress_ratio() - 0.03).abs() < 0.01);
}

#[test]
fn test_timer_completed() {
    let mut timer = Timer::default();
    timer.reset(1);
    assert!(!timer.tick());
    assert!(timer.tick());
}

#[test]
fn test_app_new() {
    let app = App::new();
    assert_eq!(app.state(), AppState::Menu);
    assert_eq!(app.selected_index(), 0);
    assert!(app.configs().len() == 3);
}

#[test]
fn test_app_enter_task_input_clears_state() {
    let mut app = App::new();
    app.enter_task_input();
    assert_eq!(app.state(), AppState::TaskInput);
}

#[test]
fn test_app_next_option() {
    let mut app = App::new();
    let initial = app.selected_index();
    app.next_option();
    assert_eq!(app.selected_index(), initial + 1);
}

#[test]
fn test_app_prev_option_wraps() {
    let mut app = App::new();
    app.prev_option();
    assert_eq!(app.selected_index(), app.configs().len() - 1);
}

#[test]
fn test_app_start_session() {
    let mut app = App::new();
    app.enter_task_input();
    app.add_char_to_task('T');
    app.add_char_to_task('e');
    app.add_char_to_task('s');
    app.add_char_to_task('t');
    app.start_session();
    assert_eq!(app.state(), AppState::Running);
}

#[test]
fn test_app_start_session_empty_keeps_state() {
    let mut app = App::new();
    app.enter_task_input();
    app.start_session();
    assert_eq!(app.state(), AppState::TaskInput);
    assert!(app.get_input_error().is_some());
}

#[test]
fn test_input_error_message() {
    let error = InputError::Empty;
    assert_eq!(error.message(), "Task name cannot be empty!");
    assert!(error.has_error());
}

#[test]
fn test_input_error_default() {
    let error = InputError::default();
    assert!(!error.has_error());
    assert_eq!(error.message(), "");
}

#[test]
fn test_input_error_none_has_no_error() {
    let error = InputError::None;
    assert!(!error.has_error());
    assert_eq!(error.message(), "");
}

#[test]
fn test_input_error_empty_has_error() {
    let error = InputError::Empty;
    assert!(error.has_error());
    assert_eq!(error.message(), "Task name cannot be empty!");
}

#[test]
fn test_app_error_cleared_on_start() {
    let mut app = App::new();
    app.enter_task_input();
    app.add_char_to_task('T');
    app.start_session();
    assert!(app.get_input_error().is_none());
}

#[test]
fn test_app_toggle_pause() {
    let mut app = App::new();
    app.enter_task_input();
    app.add_char_to_task('T');
    app.start_session();
    assert_eq!(app.state(), AppState::Running);
    app.toggle_pause();
    assert_eq!(app.state(), AppState::Paused);
    app.toggle_pause();
    assert_eq!(app.state(), AppState::Running);
}

#[test]
fn test_app_quit() {
    let mut app = App::new();
    assert!(!app.should_quit());
    app.quit();
    assert!(app.should_quit());
}

#[test]
fn test_app_enter_menu_clears_task() {
    let mut app = App::new();
    app.enter_task_input();
    app.add_char_to_task('T');
    app.add_char_to_task('e');
    app.add_char_to_task('s');
    app.add_char_to_task('t');
    app.enter_menu();
    assert!(app.task_name().is_empty());
    assert!(app.get_input_error().is_none());
}

#[test]
fn test_app_start_session_clears_error() {
    let mut app = App::new();
    app.enter_task_input();
    app.set_input_error(InputError::Empty);
    app.add_char_to_task('T');
    app.start_session();
    assert!(app.get_input_error().is_none());
    assert_eq!(app.state(), AppState::Running);
}

#[test]
fn test_phase_default() {
    let phase = Phase::default();
    assert_eq!(phase, Phase::Work);
}

#[test]
fn test_session_config() {
    let config = SessionConfig {
        label: String::from("Test"),
        work_duration_min: 25,
        break_duration_min: 5,
    };
    assert_eq!(config.work_duration_min, 25);
    assert_eq!(config.break_duration_min, 5);
}
