use nrgaccounts::calc::{ calculate, Tariffs };
use nrgaccounts::readings::find_change;
use nrgaccounts::console_input::get_reading_pair;

fn main() {

    let tariffs = Tariffs {
        import: 0.25752,
        export: 0.07135,
    };

    println!("");
    let pair = get_reading_pair();
    let changes = find_change(&pair); 
    println!("");
    println!("Report for {} days.", pair.days_spanned());
    let calculation = calculate(changes, tariffs);
    println!("{}", calculation);
}
