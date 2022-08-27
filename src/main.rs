use std::io;

fn main() {
    println!("Пожалуйста, введите нужное кол-во чисел Фибоначчи");

    let mut need_num: String = String::new();
    io::stdin()
        .read_line(&mut need_num)
        .expect("Не удалось прочесть пользовательский ввод")
    ;
    let need_num: u32 = need_num
        .trim()
        .parse()
        .expect("Введённое значение должно быть положительным целым числом")
    ;

    let mut count: u32 = 0;
    let mut previous_num: u32 = 0;
    let mut current_num: u32 = 1;

    while count < need_num {
        println!("{previous_num}");

        let next_num: u32 = fibonacci_num(previous_num, current_num);

        previous_num = current_num;
        current_num = next_num;
        count += 1;
    };
}

fn fibonacci_num(a: u32, b: u32) -> u32 {
    a + b
}
