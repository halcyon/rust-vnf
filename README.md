Vertica Native File Crate
=========================

This crate creates [Vertica Native Format](https://www.vertica.com/docs/9.2.x/HTML/Content/Authoring/AdministratorsGuide/BinaryFilesAppendix/CreatingNativeBinaryFormatFiles.htm
) (VNF) files. VNF files are optimized to load large amounts of data efficiently into a Vertica database.

![Overview](./doc/vnf.svg)

FileHeader::new(Vec[Types]) -> FileHeader
RowData::new(Vec[Types], Vec[Data]) -> RowData

VNF {
  Vec[Types]
}

Impl VNF {
  fn(&self, RowData) -> Vec[&u8]
}
