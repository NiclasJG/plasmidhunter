export interface resultData{
    annotation: annotationInterface
    hits: Array<sample>
}


interface annotationInterface{
    features: Array<featureData>,
    genome: genomeData,
    run: runData,
    sequences: Array<sequencesData>,
}


interface featureData{
    aa: string,
    aa_hexdigest: string,
    contig: string,
    db_xrefs: Array<String>,
    frame: Number,
    gene: string,
    genes: Array<String>,
    hypothetical: boolean,
    id: string,
    ips: Object,
    locus: string,
    nt: string,
    product: string,
    psc: Object,
    pscc: Object,
    rbs_motif: string,
    seq_stats: {
        isoelectric_point: Number,
        molecular_weight: Number
    },
    start: Number,
    start_type: string,
    stop: Number,
    strand: string,
    truncated: string,
    type: string,
    ups: Object
}

interface genomeData{
    complete: boolean,
    genus: string,
    gram: string,
    species: string,
    strain: string,
    translation_table: string
}

interface runData{
    end: string,
    start: string
}

interface sequencesData{
    complete: boolean,
    description: string,
    id: string,
    length: Number,
    orig_description: string,
    orig_id: string,
    nt: string,
    topology: string,
    type: string
    simple_id: string
}

interface sample{
    metadata: metaData,
    contig: Array<contig>,
    name: string
}

interface metaData{
    accession: string,
    "analysis-completed": string,
    biosample: string,
    "collection-date": Date,
    "environment-biome": string,
    "environment-feature": string,
    "environment-material": string,
    "geo-loc-name": string,
    "host-tax-id": string,
    "last-update": string,
    latitude: string,
    longitude: string,
    "sample-alias": string,
    "sample-desc": string,
    "sample-metadata": Array<{
        key: string,
        unit: string,
        value: string,
    }>
    "sample-name": string,
    species: string
}

interface contig {
    run_id: string,
    plasmid: string
    contig_id: string
    contig_start: string
    contig_end: string
    contig_hit_length: string
    contig_full_length: string
    coverage: string
    per_identity: string
    num_identity: string
    strand: string
    plasmid_start: string
    plasmid_end: string
    plasmid_len: string
    evalue: string
    bitscore: string
}



/* 
 export interface resultData {
        plasmid_name: string
        plasmid_seq: string
        hits: Array<{
            dataset: string
            geolocation: {
                location: string
                longitude: string
                latitude: string
            }
            seqMethod: string
            sampleOrigin: string
            collectionDate: string
            biosample: string
            mgnifySample: string
            contigs: Array<contig>
        }>
    } */
    