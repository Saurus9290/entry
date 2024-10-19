fn entry(age:i32) -> Result<(),String> {

    if age>=21{

        Ok(())
    }else {
        Err("not allowed".to_owned())
    }
}




fn main() {

    let entry = entry(28);
    match entry {
        Ok(d)=> println!("{:?}",d),
        Err(e) => println!("{}",e)
    }
}