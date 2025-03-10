use std::{
    io::{self, Error},
    thread,
    time::Duration,
};

pub(crate) fn mili() -> Result<(), std::io::Error> {
    let mut delay = String::new();
    println!("ask the user to enter a vlaue for the delay");
    io::stdin()
        .read_line(&mut delay)?;
    //  let delay = "delay".parse::<u64>();
    //  assert!(delay.is_err());
    //  let delay:u64 = std::result::Result::(delay);
    let value = match delay.trim().parse::<u64>() {
        Ok(num) => num,
        Err(e) => {
            // Handle the error here
            // For example, return a default value or display an error message
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid input: {}", e),
            ));
        }
    };
  
    thread::sleep(Duration::from_millis(value));
    Ok(())
}
