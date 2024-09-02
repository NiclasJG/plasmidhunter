interface resultData {
    plasmid_name: String
    plasmid_seq: String
    hits: Array<{ dataset: String; contigs: Array<contig> }>
}

interface contig {
    plasmid: String
    contig: String
    contigstart: String
    contigend: String
    contiglength: String
    coverage: String
    identity: String
    alignmentlength: String
    strand: String
    plasmidstart: String
    plasmidend: String
    plasmidlength: String
}

export { resultData }