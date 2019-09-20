use nrgaccounts::calc::calculate;
use nrgaccounts::readings::find_change;
use nrgaccounts::console_input::get_reading_pair;

fn main() {
    println!("");
    let pair = get_reading_pair();
    let changes = find_change(pair); 
    let calculation = calculate(changes);
    println!("\n{}", calculation);
}
