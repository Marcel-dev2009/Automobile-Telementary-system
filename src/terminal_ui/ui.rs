use inquire::{InquireError, Select};
use crate::terminal_ui::terminal_theme::_burgundy_theme;
pub fn run_menu (){
let parent_companies:Vec<&str> = vec!["Mercedes-Benz","Bayerische Motoren Werke"];

 let result: Result<&str, inquire::InquireError> = Select::new("Choose Parent Company!", parent_companies).with_render_config(_burgundy_theme()).prompt();
 match result {
  Ok(res) => {
   if res == "Mercedes-Benz" {
    let _ = choose_mercedes_model();    
   } else if res == "Bayerische Motoren Werke" {
     let _ = choose_bmw_model();
   } else {
    println!("No parent Company chosen");
   }
  }
  Err(_) => {
  println!("Prompt was cancelled or an error occured!");
  }
 }
}

fn choose_mercedes_model() -> Result<(), InquireError>{
    let mercedes_models:Vec<&str> = vec!["Mercedes-Benz S-Class", "Mercedes-Benz C-Class", "Mercedes-Benz E-Class", "Mercedes-Benz GLC", "Mercedes-Benz CLA"];
    let result = Select::new("Choose a model", mercedes_models).prompt()?;
    if result == "Mercedes-Benz S-Class"{
     println!("You chose {}", result);
    } else if result == "Mercedes-Benz C-Class" {
        println!("You chose {}", result);
    } else if result == "Mercedes-Benz E-Class" {
     println!("You chose {}", result);
    } else if result == "Mercedes-Benz GLC"  {
     println!("You chose {}", result);
    } else if result == "Mercedes-Benz CLA" {
      println!("You chose {}", result);
    } else {
        println!("No Mercedes model was chosen!");
    }
     Ok(())
    }
 fn choose_bmw_model() -> Result<(), InquireError>{
   let bmw_models:Vec<&str> = vec!["BMW 3 Series", "BMW 5 Series", "BMW X3 (Luxury SUV)", "BMW X5 (Midsize Luxury SUV)", "BMW 7 Series (Full-Size Flagship Sedan)"];
    let result = Select::new("Choose a model", bmw_models).prompt()?;
    if result == "BMW 3 Series"{
     println!("You chose {}", result);
    } else if result == "BMW 5 Series" {
        println!("You chose {}", result);
    } else if result == "BMW X3 (Luxury SUV)" {
     println!("You chose {}", result);
    } else if result == "BMW X5 (Midsize Luxury SUV)" {
     println!("You chose {}", result);
    } else if result == "BMW 7 Series (Full-Size Flagship Sedan)" {
      println!("You chose {}", result);
    } else {
        println!("No BMW model was chosen!");
    }
     Ok(())
    }