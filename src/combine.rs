use crate::preparevariant::preparevariant;
use crate::samread::samread;
use crate::varstruct::VcfSam;
use std::error::Error;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub async fn combinevariant(
    pathvariant: String,
    pathsam: String,
) -> Result<Vec<VcfSam>, Box<dyn Error>> {
    let variantstruct = samread(pathsam).await.unwrap();
    let samstruct = preparevariant(pathvariant).await.unwrap();
    let mut samvcfcombined: Vec<VcfSam> = Vec::new();
    for i in samstruct.iter() {
        for val in variantstruct.iter() {
            let mut posstring: Vec<String> = Vec::new();
            if i.pos <= val.refstart && i.pos <= val.refend {
                posstring.push(val.seq.clone());
            }
            samvcfcombined.push(VcfSam {
                chrom: i.chrom.clone(),
                pos: i.pos.to_string(),
                id: i.id.clone(),
                refallele: i.refallele.clone(),
                altallele: i.altallele.clone(),
                quality: i.quality.to_string(),
                filter: i.filter.clone(),
                seqvec: posstring,
            })
        }
    }
    Ok(samvcfcombined)
}
