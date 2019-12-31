extern crate tobj;

use std::fs::File;
use std::io::Write;
use std::path::Path;

fn calculate_bounding_box(mesh: &tobj::Mesh) -> (f32, f32, f32, f32, f32, f32) {
    let mut min_x = std::f32::MAX;
    let mut min_y = std::f32::MAX;
    let mut min_z = std::f32::MAX;
    let mut max_x = std::f32::MIN;
    let mut max_y = std::f32::MIN;
    let mut max_z = std::f32::MIN;

    for i_idx in 0..mesh.indices.len() {
        let f_idx = mesh.indices[i_idx];

        let x_idx = (3 * f_idx) as usize;
        let y_idx = (3 * f_idx + 1) as usize;
        let z_idx = (3 * f_idx + 2) as usize;

        let x = mesh.positions[x_idx];
        let y = mesh.positions[y_idx];
        let z = mesh.positions[z_idx];

        if x < min_x {
            min_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if z < min_z {
            min_z = z;
        }

        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if z > max_z {
            max_z = z;
        }
    }

    return (min_x, min_y, min_z, max_x, max_y, max_z);
}

fn write_flat_no_indices(original_filename: &str, models: &Vec<tobj::Model>) {
	let destination = original_filename.to_owned() + ".smodel";
    let mut output =
        File::create(&destination).expect("Unable to create file");
    println!("Writing to {}", destination);

    // @Todo improvement, this can be done in a single line
    let mut total_vectors = 0;
    let mut total_sub_models = 0;
    for m in models.iter() {
        let mesh = &m.mesh;
        total_vectors += mesh.indices.len();
        total_sub_models += 1;
    }

    write!(&mut output, "? 2.0 {} {} {}\n", original_filename, total_vectors, total_sub_models).unwrap();
    
    for (_i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        let total_vertices = mesh.indices.len();
        write!(
            &mut output,
            "% {}\n", total_vertices).unwrap();
    }

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("model[{}].name = \'{}\'", i, m.name);
        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);
        println!("model[{}].indices.len: {}", i, mesh.indices.len());

        let total_vertices = mesh.indices.len();

        let has_normals = !mesh.normals.is_empty();
        let has_texcoords = !mesh.texcoords.is_empty();

        let (min_x, min_y, min_z, max_x, max_y, max_z) = calculate_bounding_box(&mesh);

        write!(
            &mut output,
            "> {} {} {} {} {} {} {} {} {} {}\n",
            m.name,
            total_vertices,
            has_normals as i32,
            has_texcoords as i32,
            min_x,
            min_y, 
            min_z,
            max_x,
            max_y,
            max_z
        )
        .unwrap();

        // @Incomplete currently always writing normals and uvs even if they dont exist
        for i_idx in 0..mesh.indices.len() {
            let f_idx = mesh.indices[i_idx];

            let x_idx = (3 * f_idx) as usize;
            let y_idx = (3 * f_idx + 1) as usize;
            let z_idx = (3 * f_idx + 2) as usize;
            let u_idx = (2 * f_idx) as usize;
            let v_idx = (2 * f_idx + 1) as usize;

            write!(
                &mut output,
                "{} {} {}",
                mesh.positions[x_idx], mesh.positions[y_idx], mesh.positions[z_idx]
            )
            .unwrap();

            if has_texcoords {
                write!(
                    &mut output,
                    " {} {}",
                    mesh.texcoords[u_idx], mesh.texcoords[v_idx]
                )
                .unwrap();
            } else {
                write!(&mut output, " {} {}", 0, 0).unwrap();
            }

            if has_normals {
                //println!("{} {} {}",
                //    mesh.normals[x_idx], mesh.normals[y_idx], mesh.normals[z_idx]);
                write!(
                    &mut output,
                    " {} {} {}\n",
                    mesh.normals[x_idx], mesh.normals[y_idx], mesh.normals[z_idx]
                )
                .unwrap();
            } else {
                write!(&mut output, " {} {} {}\n", 0, 0, 0).unwrap();
            }
        }
    }
}

fn generate(path: &str) {
    
    let model = tobj::load_obj(&Path::new(&path));
    assert!(model.is_ok());

    let (models, _materials) = model.unwrap();

    // tobj::print_model_info(&models, &materials);
    if true {
		println!("Parsing {}", &path);    	
        write_flat_no_indices(&path, &models);
    }
}

fn main() {
    println!("Starting!");

    generate("../assets/cube.obj");
    generate("../assets/sphere.obj");
    // generate("../assets/paddle.obj");
    // generate("../assets/cube_deformed.obj");
    generate("../assets/plane.obj");

}
