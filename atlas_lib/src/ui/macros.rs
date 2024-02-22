#[macro_export]
macro_rules! update_enum {
    ($e:expr, $new:expr) => {
        if $e.self_as_index() != $new {
            $e = $e.index_as_self($new);
        }
    };
}
