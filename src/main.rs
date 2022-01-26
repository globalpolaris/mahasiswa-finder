extern crate reqwest;

use serde::Deserialize;
use serde::Serialize;

mod lib;
use lib::cli::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub mahasiswa: Vec<Mahasiswa>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mahasiswa {
    pub text: String,
    #[serde(rename = "website-link")]
    pub website_link: String,
}

fn main() {
    let app = Cli::new();
    let arg = app.get_arg();
    let name = arg.value_of("detail").unwrap();
    let mhs = get_mahasiswa(name);
    match mhs {
        Ok(mhs) => {
            for i in 0..mhs.mahasiswa.len() {
                let first_slice: Vec<&str> = mhs.mahasiswa[i].text.split(",").collect();
                let second_slice: Vec<&str> = mhs.mahasiswa[i].text.split("(").collect();
                let nama = second_slice[0];
                let third_slice: Vec<&str> = second_slice[1].split(")").collect();
                let colon_slice_univ: Vec<&str> = first_slice[1].split(" : ").collect();
                let colon_slice_prodi: Vec<&str> = first_slice[2].split(": ").collect();
                let nim = third_slice[0];
                let univ = colon_slice_univ[1];
                let prodi = colon_slice_prodi[1];

                let data = format!(
                    concat!(
                        "\n{i}\n",
                        "   Nama: {nama}\n",
                        "   NIM: {nim}\n",
                        "   Universitas: {univ}\n",
                        "   Prodi: {prodi}\n",
                        "   Link: https://pddikti.kemdikbud.go.id{link}"
                    ),
                    i = i + 1,
                    nama = nama,
                    nim = nim,
                    univ = univ,
                    prodi = prodi,
                    link = mhs.mahasiswa[i].website_link
                );
                println!("{}", data);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

#[tokio::main]
async fn get_mahasiswa<'a>(name: &str) -> Result<Root, Box<dyn std::error::Error>> {
    let url = format!("https://api-frontend.kemdikbud.go.id/hit_mhs/{}", name);
    let res = reqwest::get(&url).await?;
    let root: Root = res.json().await?;
    Ok(root)
}
