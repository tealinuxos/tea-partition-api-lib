use duct::{cmd, Expression};
use users::get_current_uid;


pub fn get_list_json_general() -> String {
    
    let parted = {
        let expression: Expression;
        if get_current_uid() != 0 {
            expression = cmd!("sudo", "parted", "-lj");
        } else {
            expression = cmd!("parted", "-lj");
        }

        expression
    };

    let parted = parted.read().expect("none");

    parted
}
