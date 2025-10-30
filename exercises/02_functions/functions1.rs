// TODO: Add some function with the name `call_me` without arguments or a return value.

#[allow(non_upper_case_globals)]
const call_me: fn() = {
    fn func() {}
    func
};

fn main() {
    call_me(); // Don't change this line
}
