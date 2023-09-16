use std::collections::HashMap;

fn main() {


    zadanie1();
    zadanie2();
    zadanie3();
    zadanie4();
    zadanie5();
    matrix();
    zadanie6();
    zadanie7();
    zadanie8();
}

fn zadanie9(){

}

// fn power(){

// }

fn zadanie8(){
    println!("zadanie8");
    matrix()
}

fn zadanie7(){
    println!("zadanie7");
    let mut results_map: HashMap<i32, i32> = HashMap::new();

    let n = 0;
    let res = fibbonacci(n, &mut results_map);
    let res2 = naive_fibbonacci(n);
    println!("result of n: {n} = {res}");
    println!("result of n: {n} = {res2}");

    for i in 0..10{
        let mut results_map: HashMap<i32, i32> = HashMap::new();
        println!("");
        let res = fibbonacci(i, &mut results_map);
        let res2 = naive_fibbonacci(i);
        println!("result of n: {i} = {res}");
        println!("result of n: {i} = {res2}");
    }
}

fn fibbonacci(n: i32, results_map: &mut HashMap<i32, i32>) -> i32{
    if n < 1 { return 0 }
    if n == 1 { return 1 }
    if results_map.contains_key(&n) {
        return *results_map.get(&n).unwrap();
    }
    let res = fibbonacci(n-1, results_map) + fibbonacci(n-2, results_map);
    results_map.insert(n, res);
    return res;
}


fn naive_fibbonacci(n: i32) -> i32{
    if n < 1 {return 0};
    if n == 1 {return 1};
    
    return naive_fibbonacci(n-1) + naive_fibbonacci(n-2);
}

fn zadanie6(){
    println!("zadanie6");

    println!("initial money after {} trips {}", 2, find_initial_money(2));
    println!("initial money after {} trips {}", 3, find_initial_money(3));
    

}

fn find_initial_money(n: i32) -> f64{

    let mut initial_money = 0.;
    for _i in 0..n{
        initial_money = 0.5 * (initial_money + 12.)
    }
    return initial_money;
}

fn matrix(){
    for i in 1..11{
        for j in 1..11{
            print!("{}\t", i * j);
        }
        println!()
    }
}

fn zadanie5(){
    fn find_amount_of_matching_numbers(n: i32) -> usize{
        return (1..(n+1))
            .filter(|x| x % 7 == 0)
            .filter(|x| (2..7).all(|y| x % y == 1)).count();
    }

    let amount_of_matching_numbers = find_amount_of_matching_numbers(100);
    println!("amount of matching numbers: {amount_of_matching_numbers}");
    println!("amount of matching numbers: {}", find_amount_of_matching_numbers(2000));

}

fn zadanie4(){

    fn find_number() -> Option<i32>{
        return (1..1000)
        .filter(|x| x % 7 == 0)
        .find(|x| (2..7).all(|y| x % y == 1) )
    }

    let res;
    match find_number(){
        Some(v) => {
            res = v;
        },
        None => {
            println!("such number was not found");
            return;
        }
    };
    println!("{res}");
}


fn zadanie3(){
    println!("zadanie3");

    fn f1(n: i64) -> i64{
        let mut s: i64 = 1;
        for k in 1..n{
            s *= k;
        }
        return s;
    }

    fn f2(n: i64) -> i64{
        let mut s: i64 = 1;
        for k in 1..n{
            s *= 2 * k + 1;
        }
        return s;
    }

    fn f3(n: i64) -> f64{
        let mut s: f64 = 1.;
        for k in 1..n{
            let k_as_f = k as f64;
            s *= 1. / (k_as_f * (k_as_f + 1.));
        }
        return s;
    }

    fn f4(n: i64) -> f64{
        let mut s: f64 = 1.;
        for k in 1..n{
            let k_as_f = k as f64;
            s *= 1. / (k_as_f * k_as_f);
        }
        return s;
    }

    fn f5(n: i64) -> i64{
        let mut s: i64 = 1;
        for k in 1..n{
            s *= k.pow(3);
        }
        return s;
    }

    let n = 5;
    println!("f1({n}) = {}", f1(n));
    println!("f2({n}) = {}", f2(n));
    println!("f3({n}) = {}", f3(n));
    println!("f4({n}) = {}", f4(n));
    println!("f5({n}) = {}", f5(n));
    


}




fn zadanie2(){
    println!("zadanie2");
    
    
    fn funny_sum(n: i32) -> i32{

        let mut sum = 0;
        print!("1 + ");
        for i in 2..(n+1){
            let number_as_string = multiply_string( i.to_string(), i);
            // println!("number as string: {number_as_string}");
            print!("{number_as_string} + ");
            sum += number_as_string.parse::<i32>().unwrap();
        }

        println!("");
        return 1 + sum;
    }

    fn multiply_string(s: String, n: i32) -> String{
        
        let mut res = s.clone();
        for _i in 1..n{
            res = format!("{}{}", res, s);
        }

        return res;
    }
    
    println!("funny sum({}) = {}", 4, funny_sum(4));
    println!("funny sum({}) = {}", 6, funny_sum(6));
}


fn zadanie1(){
    fn print_decrypt_result(text: &String, shift_count: u8, shift_right: bool){
        let custom_text = if shift_right {"right"} else {"left"};
        let decrypted_text = decrypt(text, shift_count, shift_right);
        println!("decrypted with shift count: {shift_count} to the {custom_text} = {decrypted_text}")
    }
    
    fn decrypt(text: &String, shift_count: u8, shift_right: bool) -> String{
    
        let mut decrypted_test = "".to_string();
        for c in text.chars(){
            if char::is_whitespace(c){
                decrypted_test.push(c);
                continue;
            }
            let char_as_number = c as u8;
            let shifted_char = (if shift_right {char_as_number + shift_count} else {char_as_number - shift_count} )as char;
            decrypted_test.push(shifted_char );
        }
    
        return decrypted_test;
    }

    let text = "Rmgi$ he}%".to_string();

    println!("ecrypted text is: {text}");

    for i in 1..10{
        print_decrypt_result(&text, i, true);
    }

    for i in 1..10{
        print_decrypt_result(&text, i, false);
    }
}


