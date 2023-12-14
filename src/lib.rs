use std::io;
use binaryfile::BinaryReader;
use std::collections::HashMap;

pub struct CSVReader {
    pub reader: BinaryReader,
    pub header: Vec<String>,
}

impl CSVReader {
    pub fn open(filepath: &str) -> Self {
        Self {
            reader: BinaryReader::open(filepath).unwrap(),
            header: Vec::new(),
        }
    }
    pub fn setup(&mut self) {
        let header = self.readline();
        self.header = header;
    }

    pub fn parse(&mut self, buf: Vec<u8>) -> Vec<String> {
        let linedat:String;
        if sjis::is_sjis(&buf) {
            linedat = sjis::decode(buf);   
        } else {
            linedat = String::from_utf8(buf).unwrap();
        }
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(linedat.as_bytes());

        let records = rdr.records();
        let mut columns = Vec::new();

        for record in records.into_iter() {
            let record = record.unwrap();
            for field in &record {
                columns.push(field.to_owned());
            }
        }
        columns
    }

    pub fn readline(&mut self) -> Vec<String> {
        let buf = self.reader.next().unwrap().unwrap();
        self.parse(buf)
    }
    pub fn keys(&self) -> impl Iterator<Item = String> {
        self.header.iter().enumerate().map(|(i, k)| k.clone())
    }
}

impl Iterator for CSVReader {
    type Item = Result<HashMap<String, String>, io::Error>;

    fn next( &mut self ) -> Option<Self::Item> {
        let mut data: HashMap<String, String> = HashMap::new();
        match self.reader.next() {
            Some(Ok(line)) => {
                let row = self.parse(line);

                self.header.iter().enumerate().any(|(i, k)| {
                    data.insert(k.to_string(), row.get(i).unwrap().to_string());
                    false
                });
                Some( Ok( data ) )
            }
            Some(Err(e)) => Some(Err(e)),
            None => None,
        }
    }
}


#[cfg(test)]
mod tests {

    use crate::CSVReader;

    #[test]
    #[ignore]
    fn csv_read_test() {

        let mut cr = CSVReader::open("shipping_test_data.csv");
        cr.setup();
        
        for line in cr {
            println!("{:?}", line.unwrap());
            //println!("{}", line.unwrap().get("shipping").unwrap());
        }
    }

    #[test]
    fn parse_test() {
        let mut cr = CSVReader::open("test.csv");
        cr.setup();

        for line in cr {
            let line = line.unwrap();

            //let numval:i32 = line["number"].parse().unwrap();
            let numval:i32 = line.get("number").unwrap().parse().unwrap();

            println!("string: {:8}, number: {:06}", line["string"], numval);
        }
    }

}
