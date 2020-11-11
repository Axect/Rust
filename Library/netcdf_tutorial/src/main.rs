extern crate netcdf;

fn main() {
    // Write
    let f = netcdf::test_file_new("crabs.nc");
    let mut file = netcdf::create(&f).unwrap();

    let dim_name = "ncrabs";
    file.root.add_dimension(dim_name, 10).unwrap();

    let var_name = "crab_coolness_level";
    let data: Vec<i32> = vec![42; 10];

    file.root.add_variable(
        var_name,
        &vec![dim_name.to_string()],
        &data
    ).unwrap();

    // Read
    let file = netcdf::open("testout/crabs.nc").unwrap();
    let var = file.root.variables.get("crab_coolness_level").unwrap();
    let data_read: Vec<i32> = var.get_int(false).unwrap();
    println!("{:?}", data_read);
}
