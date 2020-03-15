
#[macro_export(local_inner_macros)]
macro_rules! json_object {
    ($($obj:tt)+) => {
        json_object_internal!($($obj)+)
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! json_object_internal {
    (@map $obj:ident () () ()) => {};

    (@map $obj:ident [$($key:tt)+] ($val:expr) , $($rest:tt)*) => {
        $obj.insert($($key)+, $val);
        json_object_internal!(@map $obj () ($($rest)*) ($($rest)*));
    };

    (@map $obj:ident [$($key:tt)+] ($val:expr)) => {
        $obj.insert($($key)+, $val);
    };

    (@map $obj:ident ($($key:tt)+) (: $val:expr, $($rest:tt)*) $copy:tt) => {
        json_object_internal!(@map $obj [$($key)+] ($val), $($rest)*);
    };

    (@map $obj:ident ($($key:tt)+) (: $val:expr) $copy:tt) => {
        json_object_internal!(@map $obj [$($key)+] ($val));
    };

    (@map $obj:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        json_object_internal!(@map $obj ($key) (: $($rest)*) (: $($rest)*));
    };

    (@map $obj:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        json_object_internal!(@map $obj ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    ({ $($tt:tt)+ }) => {
        {
            let mut map = std::collections::HashMap::new();
            json_object_internal!(@map map () ($($tt)+) ($($tt)+) );
            map
        }
    };
}

#[test]
fn test_cookie_map_macro() {
    use std::collections::HashMap;

    let cookie = json_object!({
        "cpp": "hi",
        "rust": "hello world!",
    });

    let mut map = HashMap::new();
    map.insert("cpp", "hi");
    map.insert("rust", "hello world!");

    assert_eq!(cookie, map);
}
