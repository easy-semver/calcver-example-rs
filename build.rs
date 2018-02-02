extern crate calcver;


fn main() {
    let version = calcver::run(".calcver.yml",false,true);


    println!("Version udpated: {}",version); 
}