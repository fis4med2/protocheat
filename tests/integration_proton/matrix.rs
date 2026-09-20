use protocheat_core::builtin_matrix;

#[test]
fn matrix_loads() {
    assert!(!builtin_matrix().is_empty());
}
