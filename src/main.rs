extern crate rand;

use rand::Rng;

fn main() {
    let mut total = 0;
    let mut switch_success = 0;
    let mut stay_success = 0;
    for _ in 0..1000000 {
        // setting up the door
        let doors_count = 3;
        let car = rand::thread_rng().gen_range(0, doors_count);

        // contestant picks a door
        let guess = rand::thread_rng().gen_range(0, doors_count);

        // presenting an alternate door to contestant
        let mut done = false;
        let mut montys_temptation = 0;
        if guess == car {
            while !done {
                montys_temptation = rand::thread_rng().gen_range(0, doors_count);
                if montys_temptation != guess {
                    done = true
                }
            }
        } else {
            montys_temptation = car;
        }

        // checking if stay or switching leads to success
        total += 1;
        if montys_temptation == car {
            switch_success += 1;
        } else if guess == car {
            stay_success += 1;
        }
    }

    println!("Stay success: {}", stay_success);
    println!("Switch success: {}", switch_success);
    println!("total: {}", total);
}
