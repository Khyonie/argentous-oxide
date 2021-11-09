use crate::component::cartridge::Cartridge;
use std::{env::current_dir, time::SystemTime};

use nfd::Response;

mod component;

// Copyright (c) 2021-2022 Hailey "Yuki_emeralis" Garrett [yukiemeralis@gmail.com]

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE
// OR OTHER DEALINGS IN THE SOFTWARE.

fn main() 
{
    let path = loop {
        let result = nfd::open_file_dialog(None, current_dir().unwrap().to_str()).unwrap();

        let filepath: String = match result
        {
            Response::Okay(f) => {
                f
            },
            Response::OkayMultiple(_) => panic!(), // Not quite sure how this can fire here. Panic if it does.
            Response::Cancel => {
                println!("Operation cancelled. Exiting...");
                std::process::exit(0);
            },
        };

        if !filepath.ends_with(".gb")
        {
            println!("Expected file to have a .gb extension.");
            continue;
        }

        break filepath;
    };
    
    let time = SystemTime::now();
    let cart: Cartridge = Cartridge::read_rom(path.as_str());

    println!("Load time: {} μs", time.elapsed().unwrap().as_micros());

    cart.read_cart_data();
}