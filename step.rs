if and else in rust 

fn pick_snack(is_hungry:bool){
  if is_hungry == true {
    println!("Time for apple");
  }else {
    println!("Time for toys");
  }
}

panic!

fn open_box(has_toy:bool) {
  if has_toy == true {
    println("Time to play");
  }else {
    panic!("no toy no play");
  }
}

putting both together 
fn can_i_ride(age:i32) {
  if age>= 5 {
    println!("big enough for roller coaster");
  }else if age>0 {
    println!("not big enough for roller coaster");
  }else {
    panic!("how do you have a negative age ?");
  }
}

fn main() {
  can_i_ride(6);
}
