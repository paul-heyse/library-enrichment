pub fn check(table: deltalake::DeltaTable) {
    let builder = table.scan_table();
    let _ = builder.get_custom_execute_handler();
}
