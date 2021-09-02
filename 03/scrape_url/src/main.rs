use std::fs;

fn main() {
  let arg: Vec<String>  = std::env::args().collect() ;
  println!("{:#?}", arg);
  let url = &arg[1];
  let output = &arg[2];
  
  println!("Fetching url: {}", url);
  let body = reqwest::blocking::get(url).unwrap().text().unwrap();

  println!("Converting html to markdown...");
  let md = html2md::parse_html(&body);

  fs::write(output, md.as_bytes()).unwrap();
  println!("Converted markdown has been saved in {}.", output);



}