
use rand_pwd::RandPwd;
use num_bigint::BigUint;
use std::{ env, error::Error };



fn main() -> Result<(), Box<dyn Error>> {

    let demands = env::args().skip(1).map(|arg| arg.parse::<BigUint>().unwrap()).collect::<Vec<_>>();

    let mut r_p;

    if demands.is_empty() {

        r_p = RandPwd::new(0, 0, 2);
        r_p.replace_data(&["1", "2"])?;
        r_p.join();
        println!("{:?}", r_p.data());
        println!("{}", r_p);

    } else {

        let ltr_cnt = demands[0].clone();
        let sbl_cnt = demands[1].clone();
        let num_cnt = demands[2].clone();

        r_p = RandPwd::new(ltr_cnt, sbl_cnt, num_cnt);

        if demands.len() == 4 {
            let unit = demands[3].clone();
            r_p.set_unit(unit)?;
        }

        r_p.join();
        println!("{}", r_p);

    }

    Ok(())

}
