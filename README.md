Vertica Native File Creator
===========================
This crate creates [Vertica Native Format](https://www.vertica.com/docs/9.2.x/HTML/Content/Authoring/AdministratorsGuide/BinaryFilesAppendix/CreatingNativeBinaryFormatFiles.htm

) (VNF) files. VNF files are optimized to load large amounts of data efficiently into a Vertica database.

![Overview](./doc/vnf.svg)

Why Rust?
---------

Rust is a high performance, multi-paradigm system programming language focused
on safe concurrency and memory management.

Rust avoids many of the C/C++ traps and pitfalls with a consistent strong
ownership model.

Rust was voted the "most loved programming language" in the Stack Overflow Developer Survey for 2016, 2017, 2018, and 2019.

API
----

	FileHeader::new(Vec[Types]) -> FileHeader

	RowData::new(Vec[Types], Vec[Data]) -> RowData

	VNF {
	  Vec[Types]
	}

	Impl VNF {
	  fn to_bytes(&self, RowData) -> Vec[&u8]
	}

Example
-------

    use vnf::VNF
    use vnf::ColumnTypes{Integer, Binary, String}

    let column_types = vec!(Integer, Binary(1), String);
    let column_values = vec!(18, 0b11111111, "test");


	let vnf = VNF::new(column_types);
	vnf.add_row(column_values);

    let bytes = Vec<u8>::from(vnf);





Notes
-----
https://www.vertica.com/docs/9.2.x/HTML/Content/Authoring/AdministratorsGuide/BinaryFilesAppendix/Example.htm
https://stackoverflow.com/questions/41756983/vertica-convert-date-format
Use Chrono
