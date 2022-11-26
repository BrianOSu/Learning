// declaring a front_of_house module as a file
mod front_of_house;

// Use the absolute path
use crate::front_of_house::hosting;
use front_of_house::serving;

fn serv_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::serv_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();

    serving::take_order();
    serving::serve_order();

    crate::back_of_house::fix_incorrect_order();

    serving::take_payment();
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
