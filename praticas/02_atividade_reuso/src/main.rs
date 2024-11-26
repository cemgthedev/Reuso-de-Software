use atividade_lib::front_of_house::hosting;
use atividade_lib::front_of_house::serving;
use atividade_lib::back_of_house;

fn main() {
    println!("Testando reuso de bibliotecas");
    hosting::add_to_wait_list();
    hosting::seat_at_table();

    serving::take_order();
    serving::serve_order();
    serving::take_payment();

    back_of_house::take_care_trash();
}
