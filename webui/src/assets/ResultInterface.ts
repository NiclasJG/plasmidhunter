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
    }
    
    interface contig {
        plasmid: string
        contig: string
        contigstart: string
        contigend: string
        contiglength: string
        coverage: string
        identity: string
        alignmentlength: string
        strand: string
        plasmidstart: string
        plasmidend: string
        plasmidlength: string
    }
