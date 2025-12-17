use crate::varstruct::Sam;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

pub async fn samread(pathfile: String) -> Result<Vec<Sam>, Box<dyn Error>> {
    let fileopen = File::open(pathfile).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut samread: Vec<Sam> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        if line.starts_with("@") {
            continue;
        }
        if !line.starts_with("@") {
            let linevec = line.split("\t").map(|x| x.to_string()).collect::<Vec<_>>();
            samread.push(Sam {
                refname: linevec[0].to_string(),
                queryname: linevec[2].to_string(),
                refstart: linevec[3].parse::<usize>().unwrap(),
                refend: linevec[4].parse::<usize>().unwrap() + linevec[9].to_string().len(),
                seq: linevec[9].to_string(),
            })
        }
    }
    let mut filewrite = File::create("samvector.txt").expect("file not present");
    for i in samread.iter() {
        writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}",
            i.refname, i.queryname, i.refstart, i.refend, i.seq
        )
        .expect("file not present");
    }
    Ok(samread)
}
