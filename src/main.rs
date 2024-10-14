fn main() {
    let input_hex : Vec<String> = std::env::args().collect();

    if input_hex.len() < 2{
        println!("No hex nums were inputed");
    }
    else if input_hex[1] == "-h" || input_hex[1] == "--help"{
        println!("
        hcv is a simple tool for converting hex numbers to numbers we normally use
        To use hcv, run the program and as args enter your hex numbers.
        Example:
        ./exec-name ff - returns 255
            ");
    }
    else{
        start_processing(input_hex);
    }
}
fn start_processing(nums: Vec<String>){
    for n in nums{
        if !valid_hex_num(&n){
            continue;
        }
        let mut final_result = 0;
        let mut pos = 0;
        let mut is_minus = false;
        for c in n.chars(){
            if c == '-' {is_minus = true;}
            pos+=1;
            final_result += hex_chr_to_num(c) as i128 * hex(n.len()as u32-pos);
        } 

        if is_minus{
            println!("{n} = -{}", final_result);
        }
        else{
            println!("{n} = {}", final_result);
        }
    }
}

fn hex(pos: u32) -> i128{
    let mut fin_res = 1;
    for _ in 0..pos{
        fin_res *= 16;
    }
    return fin_res;
}

fn valid_hex_num(num: &String) -> bool{
    for c in num.chars() {
        if hex_chr_to_num(c) == 16{
            return false;
        }
    }
    return true;
}

fn hex_chr_to_num(chr: char) -> u8{
    if chr == '1'{
        return 1;
    }
    else if chr == '2'{
        return 2;
    }
    else if chr == '3'{
        return 3;
    }
    else if chr == '4'{
        return 4;
    }
    else if chr == '5'{
        return 5;
    }
    else if chr == '6'{
        return 6;
    }
    else if chr == '7'{
        return 7;
    }
    else if chr == '8'{
        return 8;
    }
    else if chr == '9'{
        return 9;
    }
    else if chr == 'A' || chr == 'a'{
        return 10;
    }
    else if chr == 'B' || chr == 'b'{
        return 11;
    }
    else if chr == 'C' || chr == 'c'{
        return 12;
    }
    else if chr == 'D' || chr == 'd'{
        return 13;
    }
    else if chr == 'E' || chr == 'e'{
        return 14;
    }
    else if chr == 'F' || chr == 'f'{
        return 15;
    }
    else if chr == '0' || chr == '-' || chr == '#'{
        return 0;
    }
    else{
        return 16;
    }

}
