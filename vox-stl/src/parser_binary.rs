use std::fs::File;
use std::io::{Seek, SeekFrom};

use crate::fwd::{Facet, Vec3};

use byteorder::{ReadBytesExt, LittleEndian};

/// A simple parser for binary STL files.
///
/// see: https://en.wikipedia.org/wiki/STL_(file_format)#Binary_STL
///
pub fn facets_from_binary_stl(file: &mut File) -> Result<Vec<Facet>, String> {
    file.seek(SeekFrom::Start(80)).unwrap();

    let num_triangles = file.read_u32::<LittleEndian>().unwrap();

    let mut facets = vec![];

    for _ in 0..num_triangles {
        let normal = Vec3(
            [
                file.read_f32::<LittleEndian>().unwrap(),
                file.read_f32::<LittleEndian>().unwrap(),
                file.read_f32::<LittleEndian>().unwrap(),
            ]
        );

        let tri = [
            Vec3([
                file.read_f32::<LittleEndian>().unwrap(),
                file.read_f32::<LittleEndian>().unwrap(),
                file.read_f32::<LittleEndian>().unwrap(),
            ]),
            Vec3([
                file.read_f32::<LittleEndian>().unwrap(),
                file.read_f32::<LittleEndian>().unwrap(),
                file.read_f32::<LittleEndian>().unwrap(),
            ]),
            Vec3([
                file.read_f32::<LittleEndian>().unwrap(),
                file.read_f32::<LittleEndian>().unwrap(),
                file.read_f32::<LittleEndian>().unwrap(),
            ])
        ];

        facets.push(
            Facet{
                tri,
                normal
            }
        );

        file.seek(SeekFrom::Current(2)).unwrap();
    }
    Ok(facets)
}
