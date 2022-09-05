mod cffi;

extern {
    fn abs(n: cffi::c_int) -> cffi::c_int;
    fn sqrt(n: cffi::c_double) -> cffi::c_double;
    fn ctime(tm: *time_t) -> *cffi::c_char
    fn time(t: *cffi::time_t) -> cffi::time_t;
// time_t time( time_t *arg );
}

unsafe fn c_abs(n: i32) -> i32 {
    abs(n as cffi::c_int) as i32
}

unsafe fn c_sqrt(n: f64) -> f64 {
    sqrt(n as cffi::c_double) as f64
}

unsafe fn c_ctime()

fn main() {
    unsafe {
        let n: i32 = -11;
        let n_2: f64 = 11.0;
        println!("The absolute value of {} is {}", n, c_abs(n));
        println!("The square root of {} is {}", n_2, c_sqrt(n_2));

        let tm: cffi:
    }
}
