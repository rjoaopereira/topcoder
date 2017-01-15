//https://community.topcoder.com/stat?c=problem_statement&pm=2913&rd=5849
extern crate pcre;
extern crate enum_set;

use std::io::{stdin};
use enum_set::{EnumSet};
use pcre::{CompileOption, Pcre};

fn find_name(existing_names: &Vec<&str>, new_name: &str) -> bool {
    for &existing_name in existing_names.iter() {
        println!("existing_name {0} - new_name: {1} asdasd", existing_name, new_name);
        if existing_name == new_name {
            return true
        }
    }
    false
}

fn find_new_name(existing_names: &Vec<&str>, new_name: &str) -> String {
    let mut count: i32 = 0;
    let base = &new_name;
    let mut tentative = String::new();
    tentative.push_str(new_name);
    println!("contains {}", existing_names.contains(&tentative.as_str()));
    while existing_names.contains(&tentative.as_str()){
        count+=1;
        tentative.clear();
        tentative.push_str(base);
        tentative.push_str(&format!("{}", count));
    }
    tentative.clear();
    tentative.push_str("\"");
    tentative.push_str(base);
    tentative.push_str(&format!("{}", count));
    tentative.push_str("\"");
    tentative
}

fn main() {
    // Prints each argument on a separate line
    let mut compile_options: EnumSet<CompileOption> = EnumSet::new();
    compile_options.insert(CompileOption::DupNames);

    let mut re = match Pcre::compile_with_options("^(?=.{1,50}$)([a-zA-Z]+([1-9][0-9]*)*)$", &compile_options) {
        Err(err) => {
            println!("Error: The pattern could not be compiled: {}", err);
            return;
        },
        Ok(re) => re
    };

    let mut re2 = match Pcre::compile_with_options("^[a-zA-Z]{1,50}$", &compile_options) {
        Err(err) => {
            println!("Error: The pattern could not be compiled: {}", err);
            return;
        },
        Ok(re) => re
    };

    loop {

        let mut existing_names: Vec<&str>;
        println!("input existing names");
        let mut existing_names_input = String::new();
        stdin().read_line(&mut existing_names_input)
            .expect("failed to read line");

        let stripped_names = existing_names_input.replace("{", "").replace("}", "").replace(" ", "").replace("\"", "");
        let existing_names: Vec<&str> = stripped_names.split(",").collect();
        let existing_names_ref = &existing_names;
        if existing_names_ref.len() < 51 && existing_names_ref.len() > 0 {
            for existing_name in existing_names_ref {
                match re.exec(existing_name) {
                    None => {
                        panic!("{} is not valid", existing_name);
                    }
                    Some(m) =>  {
                        println!("name: {}", existing_name);
                    }
                };
            }
        }

        println!("input new name");
        let mut new_name_input = String::new();
        stdin().read_line(&mut new_name_input)
            .expect("failed to read line");
        new_name_input = new_name_input.replace("\"", "");
        match re2.exec(&new_name_input) {
            None => {
                panic!("{} is not valid", new_name_input);
            }
            Some(m) =>  {
                m;
            }
        };

        let new_name = new_name_input;

        println!("check if {} is not taken", new_name);
        if find_name(&existing_names, &new_name.trim()) {
            println!("username: {}", find_new_name(&existing_names, &new_name.trim()));
        } else {
            println!("username: {}", new_name);
        }


        break;
    }

}
