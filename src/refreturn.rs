use crate::varstruct::Sam;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 Gaurav Sablok
 codeprog@icloud.com
*/

pub async fn refreturn(refname: String) -> Result<Vec<Sam>, Box<dyn Error>> {
    let fileread = File::open("samvector.txt").expect("file not present");
    let filereadon = BufReader::new(fileread);
    let mut samevectoropen: Vec<Sam> = Vec::new();
    for i in filereadon.lines() {
        let line = i.expect("file not present");
        let linevec = line.split("\t").map(|x| x.to_string()).collect::<Vec<_>>();
        samevectoropen.push(Sam {
            refname: linevec[0].to_string(),
            queryname: linevec[1].to_string(),
            refstart: linevec[2].parse::<usize>().unwrap(),
            refend: linevec[5].parse::<usize>().unwrap(),
            seq: linevec[6].to_string(),
        })
    }
    let mut selectedvec: Vec<Sam> = Vec::new();
    for i in samevectoropen.iter() {
        if i.refname == refname {
            selectedvec.push(Sam {
                refname: i.refname.clone(),
                queryname: i.queryname.clone(),
                refstart: i.refstart,
                refend: i.refend,
                seq: i.seq.clone(),
            });
        }
    }
    Ok(samevectoropen)
}
