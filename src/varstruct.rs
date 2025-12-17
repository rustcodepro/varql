use async_graphql::SimpleObject;

#[derive(Debug, Clone, PartialOrd, PartialEq, SimpleObject)]
pub struct Sam {
    pub refname: String,
    pub queryname: String,
    pub refstart: usize,
    pub refend: usize,
    pub seq: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, SimpleObject)]
pub struct Vcf {
    pub chrom: String,
    pub pos: usize,
    pub id: String,
    pub refallele: String,
    pub altallele: String,
    pub quality: usize,
    pub filter: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, SimpleObject)]
pub struct VcfSam {
    pub chrom: String,
    pub pos: String,
    pub id: String,
    pub refallele: String,
    pub altallele: String,
    pub quality: String,
    pub filter: String,
    pub seqvec: Vec<String>,
}
