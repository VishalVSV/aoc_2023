
macro_rules! run {
    ($day: ident) => {
        let raw_data = std::fs::read_to_string(format!("./data/{}.txt", stringify!($day))).expect(&format!("Data not found for day {}", stringify!($day)));

        let time = std::time::SystemTime::now();

        let (res1, res2) = $day(raw_data);

        println!("Result:\n  Part 1: {}\n  Part 2: {}", res1, res2);
        if let Ok(elapsed) = time.elapsed() {
            println!("Time taken: {}μs", elapsed.as_micros());
        }
    };
}

#[allow(unused_macros)]
macro_rules! day {
    ($day: ident, $fn_body_p1: block) => {
        fn $day(raw_data: String) -> (String, String) {
            ($fn_body_p1(raw_data), String::new())
        }
    };
    ($day: ident, $fn_body_p1: expr, $fn_body_p2: expr) => {
        fn $day(raw_data: String) -> (String,String) {
            ($fn_body_p1(raw_data.clone()), $fn_body_p2(raw_data))
        }
    };
}

macro_rules! day_c {
    ($day: ident, $fn_body_p1: expr) => {
        fn $day(raw_data: String) -> (String,String) {
            $fn_body_p1(raw_data)
        }
    };
}

#[allow(unused_macros)]
macro_rules! time {
    ($($body:tt)*) => {
        println!("Line:{}\n{}", line!(), stringify!($($body)*));

        let start = std::time::SystemTime::now();
        $($body)*
        if let Ok(elapsed) = start.elapsed() {
            println!("Time taken: {}μs", elapsed.as_micros());
        }
    };
}