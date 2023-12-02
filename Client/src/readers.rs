use csv::ReaderBuilder;
use nalgebra::{DMatrix, DVector};
use nalgebra_sparse::{csr::CsrMatrix, CooMatrix};
use std::{error::Error, fs::File, io::BufReader, time::Instant};

pub fn create_vector_from_csv(filename: &str) -> Result<DVector<f64>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(filename)?;
    let mut record = csv::ByteRecord::new();
    let mut deserialized_records: Vec<f64> = Vec::new();

    while rdr.read_byte_record(&mut record)? {
        deserialized_records.push(record.deserialize::<f64>(None)?);
    }

    Ok(DVector::from_vec(deserialized_records))
}
