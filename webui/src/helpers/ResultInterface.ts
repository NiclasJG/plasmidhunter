export interface resultData{
    annotation: annotationInterface
    hits: Array<sample>
}

// All data received from bakta (annotation)
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

// All data received from metadata and blastn output (contigs)
interface sample{
    metadata: metaData,
    runs: Array<run>,
    name: string
}

export interface metaData{
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

interface run {
    run_id: string,
    plasmid_len: number,
    total_identity: string,
    total_coverage: string,
    contigs: Array<contig>
}

interface contig {
    contig_id: string
    contig_len: string
    plasmid_coverage: string
    contig_coverage: string
    weighted_identity: string
    alignments: Array<alignment>
}

interface alignment {
    plasmid_start: string
    plasmid_end: string
    contig_start: string
    contig_end: string
    alignment_len: string
    strand: string
    perc_identity: string
    evalue: string
    bitscore: string
}
