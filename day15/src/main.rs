use std::collections::HashMap;

fn play_one_round( tracker: &mut HashMap<u32, u32>, turn_counter: u32, last_value: u32) -> u32 {
    //println!("Turn: {} -> {}", turn_counter, last_value);
    match tracker.insert(last_value, turn_counter) {
        Some(turn) => turn_counter - turn,
        None => 0
    }
}

fn main() {
    let mut tracker : HashMap<u32, u32> = HashMap::new();
    let mut turn_counter = 0;

    let initial_values = vec![1,20,11,6,12,0];

    for value in initial_values.iter() {
        turn_counter +=1;
        tracker.insert(*value, turn_counter);
    }

    let last_call = (turn_counter..30000000).fold(*initial_values.last().unwrap(), |last_value, turn| play_one_round(&mut tracker, turn, last_value));

    println!("{}", last_call)


}
