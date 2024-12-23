export interface resultData{
    annotation: annotationInterface
    hits: Array<metaData>
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
    sequence: string,
    topology: string,
    type: string
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
    species: string,
    contigs: Array<contig>
}

interface contig {
    plasmid: string
    contig: string
    "contig start": string
    "contig end": string
    "contig length": string
    "coverage[%]": string
    "identity[%]": string
    "alignment length": string
    strand: string
    "plasmid start": string
    "plasmid end": string
    "plasmid length": string
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
    