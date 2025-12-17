use crate::varstruct::Vcf;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

pub async fn preparevariant(pathvariant: String) -> std::io::Result<Vec<Vcf>> {
    let fileopen = File::open(pathvariant).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut variantfile: Vec<Vcf> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        let linevec = line.split("\t").map(|x| x.to_string()).collect::<Vec<_>>();
        variantfile.push(Vcf {
            chrom: linevec[0].to_string(),
            pos: linevec[1].parse::<usize>().unwrap(),
            id: linevec[2].to_string(),
            refallele: linevec[3].to_string(),
            altallele: linevec[4].to_string(),
            quality: linevec[5].parse::<usize>().unwrap(),
            filter: linevec[6].to_string(),
        });
    }
    let mut filewrite = File::create("variantfiltered.txt").expect("file not present");
    for i in variantfile.iter() {
        writeln!(
            filewrite,
            "{}\t{}\t{}\t{}\t{}\t{}",
            i.chrom, i.pos, i.id, i.refallele, i.altallele, i.quality
        )
        .expect("file not present");
    }

    Ok(variantfile)
}
