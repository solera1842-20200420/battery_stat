/* 
fn main() {
   println!("Hello, world!");
}
*/

/*
report battery status
*/

// print battey status
fn main() -> Result<(), battery::Error> {
		println!("---BatteryStatus---\n");
    let manager = battery::Manager::new()?;

    for (idx, maybe_battery) in manager.batteries()?.enumerate() {
        let battery = maybe_battery?;
        println!("Battery #{}:", idx);
        println!("Vendor: {:?}", battery.vendor());
        println!("Model: {:?}", battery.model());
        println!("State: {:?}", battery.state());
        println!("Time to full charge: {:?}", battery.time_to_full());
        println!("");
    }

    Ok(())
}

