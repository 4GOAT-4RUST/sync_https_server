use std::collections::VecDeque;
pub fn upcomming_input(input: String) -> String {
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(input);
  let item = "Hello";
    for _ in 0..8 {
        let item = queue[0].clone();
        queue.pop_front();
    }
   item.to_string()
}