// use standard env library to read command line arguments
use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::process;

// Arguments struct will be used to store the arguments sent in at 
// command line by the user
struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        // Obtain the first input from user at the command line
        let f = args[1].clone();

        // Ipaddr::from_str() function takes a &str and returns Result<IpAddr, AddrParseError>
        // for reference Result enum is as follows:

/*             pub enum Result<T, E> {
                Ok(T),
                Err(E),
            }
 */

        // So below the right hand side will return a Ok(IpAddr) if user enters a 
        // Ipaddress at the command line without any flags. So the vector args[1]
        // will contain that address as string. Now with if let syntac that IpAddr 
        // binded to the variable ipaddr per the pattern Ok(ipaddr) 


        if let Ok(ipaddr) = IpAddr::from_str(&f) {

            // Note per below as structs take ownership of the data. It appears that we are 
            // passing the data which can be fully owned by the Struct
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4});

        } else {  // else will take care of the condition with &f is not a IpAddr when parsed by the IpAddr::from_str function
                 // user could have provided a flag. So the code below takes care of that condition
            let flag = args[1].clone(); 
        
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want
                \r\n    -h or -help to show this help message");
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-j") {

                // below &str method parse is based on the Rust doc as shown below

               /*  pub fn parse<F>(&self) -> Result<F, <F as FromStr>::Err>
                    where
                        F: FromStr, 
 */

                // parse converts a string slice to the type you want it based on the
                // F type. All F traits which work with this function would have implemented
                // FromStr trait. Eg in docs is as follows using the "turbofish" syntax
                // let four = "4".parse::<u32>();
                // Note u32 string "4" will be converted to u32 but since parse returns a 
                // Result enum you will get Ok(4)

                // per the above explanation we are just converting the strting to number below
                // The below match will return "s" which will be a u16 which will be assigned to 
                // threads. If error occurs, we break from here and return Err as the return type
                // from this function
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number")
                };
        
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPaddress; must be IPv4 or IPv6")
                };

                return Ok(Arguments {threads, flag, ipaddr});
            } else {
                return Err("Invalid syntax");
            }
        }

    }
}



fn main() {

    // Store the command line input into a vector using args() and collect()
    // function
    let args: Vec<String> = env::args().collect();

    // Initial test printout the args to see what it stored

    for arg in &args {
        println!("{}", arg);
    }

    // Print the vector
    println!("{:?}", args);

    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                // exit(0) - exits the program without panicking
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arhuments: {}", program, err);
                process::exit(0);

            }
        }
    );



}
