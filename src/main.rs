use std::fs::File;
use std::io;
use stl_io;
use clap::{Arg, App};

fn main() {
    let matches = App::new("stlcat")
        .version("0.1.0")
        .author("James Stanley <james@incoherency.co.uk>")
        .about("Concatenate STL files")
        .arg(Arg::with_name("FILE")
                 .min_values(1)
                 .required(true)
                 .help("A cool file"))
        .get_matches();

    let mut newmesh = Vec::new();

    let names = matches.values_of("FILE").unwrap();
    for stlfilename in names {
        let mut file = File::open(stlfilename).unwrap();
        let stl = stl_io::read_stl(&mut file).unwrap();
        for t in stl.faces {
            newmesh.push(stl_io::Triangle {
                normal: t.normal,
                vertices: [stl.vertices[t.vertices[0]], stl.vertices[t.vertices[1]], stl.vertices[t.vertices[2]]],
            });
        }
    }

    stl_io::write_stl(&mut io::stdout(), newmesh.iter()).unwrap();
}
