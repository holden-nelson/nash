#[cfg(test)]
mod tests {
    use crate::core::{EditorCore, EditorEvent, Step};

    use EditorEvent::*;

    fn buf(core: &EditorCore) -> String {
        core.view().text.to_string()
    }

    #[test]
    fn starts_empty() {
        let core = EditorCore::new();
        assert_eq!(buf(&core), "");
    }

    #[test]
    fn inserts_characters() {
        let mut core = EditorCore::new();

        assert_eq!(core.handle(Char('a')), Step::Continue);
        assert_eq!(core.handle(Char('b')), Step::Continue);
        assert_eq!(core.handle(Char('c')), Step::Continue);

        assert_eq!(buf(&core), "abc");
    }

    #[test]
    fn backspace_removes_left_of_cursor() {
        let mut core = EditorCore::new();

        core.handle(Char('a'));
        core.handle(Char('b'));
        core.handle(Backspace);

        assert_eq!(buf(&core), "a");
    }

    #[test]
    fn delete_forward_removes_right_of_cursor() {
        let mut core = EditorCore::new();

        // Build "ab", then move cursor between 'a' and 'b', then delete-forward removes 'b'.
        core.handle(Char('a'));
        core.handle(Char('b'));
        core.handle(Left);

        core.handle(Delete);

        assert_eq!(buf(&core), "a");
    }

    #[test]
    fn left_right_navigation_does_not_change_text() {
        let mut core = EditorCore::new();

        core.handle(Char('a'));
        core.handle(Char('b'));
        core.handle(Char('c'));

        core.handle(Left);
        core.handle(Left);
        core.handle(Right);

        assert_eq!(buf(&core), "abc");
    }

    #[test]
    fn home_end_navigation_does_not_change_text() {
        let mut core = EditorCore::new();

        core.handle(Char('a'));
        core.handle(Char('b'));
        core.handle(Char('c'));

        core.handle(Home);
        core.handle(End);

        assert_eq!(buf(&core), "abc");
    }

    #[test]
    fn take_on_empty_is_empty_and_clears() {
        let mut core = EditorCore::new();
        assert_eq!(core.take(), "");
        assert_eq!(buf(&core), "");
    }

    #[test]
    fn enter_completes_and_inserts_newline() {
        let mut core = EditorCore::new();

        core.handle(Char('h'));
        core.handle(Char('i'));

        assert_eq!(core.handle(Enter), Step::Completed);

        // newline should be part of buffer
        assert_eq!(buf(&core), "hi");

        // take() returns contents and clears buffer
        let taken = core.take();
        assert_eq!(taken, "hi");
        assert_eq!(buf(&core), "");
    }

    #[test]
    fn ctrl_j_completes_and_inserts_newline() {
        let mut core = EditorCore::new();

        core.handle(Char('h'));
        core.handle(Char('i'));

        assert_eq!(core.handle(CtrlJ), Step::Completed);

        assert_eq!(buf(&core), "hi\n");

        let taken = core.take();
        assert_eq!(taken, "hi\n");
        assert_eq!(buf(&core), "");
    }

    #[test]
    fn ctrl_c_aborts_and_does_not_insert_newline() {
        let mut core = EditorCore::new();

        core.handle(Char('h'));
        core.handle(Char('i'));

        assert_eq!(core.handle(CtrlC), Step::Aborted);

        // No newline added on abort
        assert_eq!(buf(&core), "hi");

        // take() can still retrieve what was typed, and clears afterwards
        let content = core.take();
        assert_eq!(content, "hi");
        assert_eq!(buf(&core), "");
    }

    #[test]
    fn can_reuse_core_for_multiple_lines() {
        let mut core = EditorCore::new();

        core.handle(Char('o'));
        core.handle(Char('n'));
        core.handle(Char('e'));
        assert_eq!(core.handle(Enter), Step::Completed);

        assert_eq!(buf(&core), "one");
        assert_eq!(core.take(), "one");
        assert_eq!(buf(&core), "");

        core.handle(Char('t'));
        core.handle(Char('w'));
        core.handle(Char('o'));
        assert_eq!(core.handle(Enter), Step::Completed);

        assert_eq!(buf(&core), "two");
        assert_eq!(core.take(), "two");
        assert_eq!(buf(&core), "");
    }
}
