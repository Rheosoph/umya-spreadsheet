use std::io::Cursor;

fn workbook() -> Vec<u8> {
    let book = umya_spreadsheet::new_file();
    let mut bytes = Vec::new();
    umya_spreadsheet::writer::xlsx::write_writer(&book, &mut bytes).unwrap();
    bytes
}

#[test]
fn reads_consistent_workbooks_eagerly_and_lazily() {
    for with_sheet_read in [true, false] {
        let book =
            umya_spreadsheet::reader::xlsx::read_reader(Cursor::new(workbook()), with_sheet_read)
                .unwrap();
        assert_eq!(book.get_sheet_count(), 1);
    }
}

#[test]
fn rejects_conflicting_headers_before_loading_workbook() {
    let mut bytes = workbook();
    let offset = zip::ZipArchive::new(Cursor::new(&bytes))
        .unwrap()
        .by_name("xl/workbook.xml")
        .unwrap()
        .header_start() as usize;
    bytes[offset + 30] ^= 1;
    for with_sheet_read in [true, false] {
        let error =
            umya_spreadsheet::reader::xlsx::read_reader(Cursor::new(&bytes), with_sheet_read)
                .err()
                .expect("inconsistent headers must fail before workbook parsing");
        assert!(error.to_string().contains("filename mismatch"), "{error}");
    }
}
