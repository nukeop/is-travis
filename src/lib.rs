use std::env;

pub fn is_travis() -> bool {    
    if let (Ok(_val1), Ok(_val2)) = (env::var("TRAVIS"), env::var("CI")) {       
        return true;
    } else {
        return false;
    }
}
