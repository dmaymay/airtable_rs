# airtable_helper
Airtable API helper for Rust

# TO DO

- Get Records // also from specified view
- Create Record function (if no rec*****) 
- Update Record function (if contains rec****) PATCH or PUT option?
- Infer types from table metadata / field types
- Print out map with fields and inferred types
- Test actually using data
- Modify data -> back to update or create record ( x type to string?)
- Merge record function ( batch into create or update according to record status) --> performUpsert merge on id??
- Recursive offset when getting records -> specify MaxRecords if user wants to limit records
- Manage Limit of 10 records when creating or updating records (Batch update always if possible)
- Delete Records -> Manage 10 limit deleting

- Manage Batch call / sec limits (specially with async)

- Token and Base validation
- Upload Attachments
- Create / update fields
- Create / update Tables
- Sync CSV Data -> Create table from csv?

- Handle linked records (automatically fetch data from linked table (recursion limit?????))

- List Bases
- Get Base Schema
- Create Bases

- Create Client (logged in with credential etc )
- Error Handling


