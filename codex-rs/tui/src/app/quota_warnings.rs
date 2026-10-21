//! Persist the local quota-warning preference before changing the running UI.

use super::App;

impl App {
    pub(super) fn set_quota_warnings_enabled(&mut self, enabled: bool) {
        if let Err(err) =
            crate::quota_warnings::save(self.local_settings.codex_home.as_path(), enabled)
        {
            self.chat_widget.add_error_message(format!(
                "Failed to save quota warning preference; setting unchanged: {err}"
            ));
            return;
        }
        self.local_settings.show_quota_warnings = enabled;
        self.chat_widget.set_quota_warnings_enabled(enabled);
        let state = if enabled { "on" } else { "off" };
        self.chat_widget.add_info_message(
            format!("Quota warnings: {state} (saved)."),
            /*hint*/ None,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::app::test_support::make_test_app;

    #[tokio::test]
    async fn quota_warning_preference_round_trips_and_preserves_other_settings() {
        let mut app = make_test_app().await;
        let path = app.local_settings.user_config_path.to_path_buf();
        std::fs::write(
            &path,
            "# keep this comment\n[notice]\nhide_rate_limit_model_nudge = true\n",
        )
        .unwrap();
        for enabled in [false, true] {
            app.set_quota_warnings_enabled(enabled);
            let text = std::fs::read_to_string(&path).unwrap();
            assert_eq!(
                text,
                "# keep this comment\n[notice]\nhide_rate_limit_model_nudge = true\n"
            );
            assert_eq!(
                crate::quota_warnings::enabled(app.local_settings.codex_home.as_path()),
                enabled
            );
            assert_eq!(app.chat_widget.quota_warnings_enabled(), enabled);
            assert_eq!(app.local_settings.show_quota_warnings, enabled);
            assert_eq!(
                crate::local_settings::LocalSettings::from(&app.config).show_quota_warnings,
                enabled
            );
        }
    }

    #[tokio::test]
    async fn quota_warning_write_failure_keeps_current_preference() {
        let mut app = make_test_app().await;
        // A file cannot contain the preference marker.
        std::fs::write(app.local_settings.user_config_path.as_path(), "").unwrap();
        app.local_settings.codex_home = app.local_settings.user_config_path.clone();
        app.set_quota_warnings_enabled(false);
        assert!(app.chat_widget.quota_warnings_enabled());
        assert!(app.local_settings.show_quota_warnings);
    }
}
