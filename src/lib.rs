#![no_std]
/// Create a curried function
/// 
/// Example:
/// 
/// You can curry an add function that adds two numbers.
/// 
/// Simplest form, without any type annotations.
/// You need to use the curried function so that
/// the rust compiler can infer the input and return
/// types for you:
/// ```
/// # use curry_macro::curry;
/// let add = curry!(|a, b| a + b);
/// assert_eq!(add(1)(2), 3);
/// ```
/// With input type annotations:
/// ```
/// # use curry_macro::curry;
/// let add = curry!(|a: i32, b: i32| a + b);
/// ```
/// With input and return type annotations and a block as function body
/// ```
/// # use curry_macro::curry;
/// let add = curry!(|a: i32, b: i32| -> i32 { a + b });
/// ```
/// The above three functions work the same:
/// ```
/// # use curry_macro:: curry;
/// # let add = curry!(|a, b| a + b);
/// // You can generate intermediate functions that are partially applied:
/// let add1_to = add(1);
/// let sum = add1_to(2);
/// assert_eq!(sum, 3);
/// // You can also can apply all arguments at once:
/// let sum = add(1)(2);
/// assert_eq!(sum, 3);
/// ```
#[macro_export]
macro_rules! curry (
   // Simplest form, without any type annotations.
    (|$first_arg:ident $(, $arg:ident )*| $function_body:expr) => {
       move |$first_arg| $(move |$arg|)* {
          $function_body
       }
    };
    // With input type annotations
    (|$first_arg:ident:$first_arg_type:ty $(, $arg:ident:$arg_type:ty )*| $function_body:expr) => {
      move |$first_arg:$first_arg_type| $(move |$arg:$arg_type|)* {
         $function_body
      }
   };
   // With input and return type annotations and a block as function body
   (|$first_arg:ident:$first_arg_type:ty $(, $arg:ident:$arg_type:ty )*| -> $ret_type:ty $function_body:block) => {
    move |$first_arg:$first_arg_type| $(move |$arg:$arg_type|)* -> $ret_type {
       $function_body
    };
   };
);
