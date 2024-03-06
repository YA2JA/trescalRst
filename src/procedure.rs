use std::{fs::{read_to_string, File, OpenOptions}, io::Write};

use crate::gpib;
pub struct Process
{
    steps:Vec<String>,
    result:Vec<String>,
}

impl Process
{
    pub fn new(file_path:&str) -> Process
    {
        let steps = read_lines(file_path);
        let result:Vec<String> = vec![];

        Process {
            steps:steps,
            result:result
        }
    }

    pub fn exec(&mut self, 
                multi:&gpib::Communication,
                power_sup:&gpib::Communication)
    {
        let steps = (&self).steps.clone();
        for step in steps
        {
            if step.is_empty() {continue};
            match &step[..1]
            {
                "*" => self.save_data(),
                "#" => {continue},
                "|" => {
                    println!("config");
                }
                _ => {
                        self.command_to_push(&step, multi, power_sup);
                    },
            }
            
        }
    }

    fn command_to_push(&mut self,
                        step:&String,
                        multi:&gpib::Communication,
                        power_sup:&gpib::Communication)
    {
        match step.starts_with("MEASure")
        {
            true => {
                self.result.push(multi.get(step.as_bytes()));
            },
            false => {
                match step.ends_with("?")
                {
                    true => self.result.push(power_sup.get(step.as_bytes())),
                    false => power_sup.set(step.as_bytes()),
                }
            }
        }
    }

    fn save_data(&mut self)
    {   
        if self.result.is_empty()
        {
            return
        };

        let mut file = match OpenOptions::new().append(true).open("output.csv") 
        {
            Ok(f) => f,
            Err(_) => File::create("output.csv").expect("Can't create file"),
        };

        for value in &self.result 
        {
                let m:f64 = value.replace("\n", "").parse().expect("msg");

                file.write_all(format!("{:.3};", m)
                            .replace(".", ",")
                            .as_bytes())
                            .expect("Can't save");
        }
        file.write_all(b"\n").expect("Can't save");
        self.result.clear()
        
    }
}

fn read_lines(filename: &str) -> Vec<String>
{
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines()
    {
        result.push(line.to_string())
    }
    return result;
}