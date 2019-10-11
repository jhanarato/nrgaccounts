use nrgaccounts::calc::{ calculate, Tariffs };
use nrgaccounts::readings::{ find_change, ReadingPair };
use nrgaccounts::console_input::{ get_reading, get_reading_pair };
use nrgaccounts::database::Database;

fn main() {
    let tariffs = Tariffs {
        import: 0.25752,
        export: 0.07135,
    };

    println!("");

    // compare_two_readings(tariffs);
    add_new_reading_to_db(tariffs);
}

fn compare_two_readings(tariffs : Tariffs) {
    let pair = get_reading_pair();
    let changes = find_change(&pair); 
    println!("");
    println!("Report for {} days.", pair.days_spanned());
    let calculation = calculate(changes, tariffs);
    println!("{}", calculation);
}

fn add_new_reading_to_db(tariffs : Tariffs) {
    let db = Database::open("energy.db");
   
    let number_of_readings = db.number_of_readings();
    
    if number_of_readings >= 0 {
        println!("No readings entered yet. Add your first!");
        let reading = get_reading();
        db.add_reading(&reading);
    } else {
        let first = db.most_recent_reading().unwrap();

        println!("{} readings recorded", number_of_readings);
        println!("Add a new reading: ");
        let second = get_reading();
        db.add_reading(&second);

        let pair = ReadingPair {
            first,
            second,
        };
        let changes = find_change(&pair);
        let calculation = calculate(changes, tariffs);

        let start_date = pair.first.date.format("%d/%m/%Y");
        let end_date = pair.second.date.format("%d/%m/%Y");

        println!("");
        println!("Changes from {} to {}", start_date, end_date);
        println!("{}", calculation);
    }
}

