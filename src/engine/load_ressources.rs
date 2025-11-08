use std::sync::Arc;
use macroquad::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::copy;
use std::error::Error;
use std::fs::{create_dir_all};

use std::path::Path;
use reqwest::blocking::Response;
use reqwest::StatusCode;



pub async fn write_texture(link: &str) -> Arc<Texture2D>{
   
    let (url,path) = extract_url_and_path(link);

    match (url.as_deref(), path.as_deref()) 
           {
            (None, None) => panic!("Invalid input: \"{link}\""),

            (None, Some(p)) =>{ 
                    println!("None Some({})",p);
                    let texture_result = load_texture(p).await;
                    let texture: Texture2D = match texture_result {
                        Ok(texture) => {
                            println!("Texture loaded successfully.");
                            texture
                        }
                        Err(e) => {
            
                            println!("Failed to load texture: {}. Loading fallback texture.", e);
          
                            load_texture("src/engine/DEFAULT_TEXTURE/missing.png").await.unwrap_or_else(|e| {
                                eprintln!("Failed to load fallback texture: {}", e);
                                panic!("No texture could be loaded.");
                            })
                        }
                    };
                    texture.set_filter(FilterMode::Linear);
                    return Arc::new(texture)
            }

            (Some(u), Some(p)) =>{
                    // if valid link and path: try to load texture. If none is found, download from URL and try again
                    println!("Some({}) Some({})",u,p);
                    let texture_result = load_texture(p).await;
                    let texture: Texture2D = match texture_result {
                        Ok(texture) => {
                            println!("Texture loaded successfully.");
                            texture
                        }
                        Err(e) => {

                            //download texture
                            println!("downloading file {u} as {p}");
                            if let Err(e) = download_file(u, p) {
                            eprintln!("Error downloading {}: {}", u, e);
                            }else{println!("download success");}



                            //try again
                            let texture_result = load_texture(p).await;
                            let texture: Texture2D = match texture_result {
                                Ok(texture) => {
                                    println!("Texture loaded successfully.");
                                    texture
                                }
                                Err(e) => {
            
                                    println!("Failed to load texture: {}. Loading fallback texture.", e);
          
                                    load_texture("src/engine/DEFAULT_TEXTURE/missing.png").await.unwrap_or_else(|e| {
                                        eprintln!("Failed to load fallback texture: {}", e);
                                        panic!("No texture could be loaded.");
                                    })
                                }
                            };
                            texture.set_filter(FilterMode::Linear);
                            return Arc::new(texture)

                        }
                    };
                    texture.set_filter(FilterMode::Linear);
                    return Arc::new(texture)
            
                   }
    
            (Some(u), None) => panic!("Some({}) None",u),
           }


   
}




fn extract_url_and_path(input: &str) -> (Option<&str>,Option<&str>) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    {
        for i in parts.iter(){
        
            println!("part:{}\n",i);
        }
    }
    if parts.len()<=1 {

        let r = parts.get(0).copied();
        return (None, r);
    }

    if parts.len()==2 {

        let mut urll: Option<&str>= None;
        if let Some(url)= parts.get(0){
            
            let url_regex = Regex::new(r"(?i)\b((?:(https?|ftp)://|www\.)[-A-Z0-9+&@#/%?=~_|!:,.;]*[-A-Z0-9+&@#/%=~_|])\b")
            .unwrap();
            
            if url_regex.is_match(url) {
                println!("successful regex match!!");
                urll= Some(url);
            }
            else{ panic!("\"{}\" does not contain a valid URL or is improperly formated\nExpected formating:\nimage.png\nor\nurl image.png\n\n", input); }
        }
        
        let r = parts.get(1).copied();
 
        return (urll, r)
    }
    println!("\"{}\" does not contain a valid URL or is improperly formated\nExpected formating:\nimage.png\nor\nurl image.png\n\n", input);
    (None, None)
}

fn download_file(url: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // Parse the output path
    let path = Path::new(output_path);

    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    // Send GET request
    let response = reqwest::blocking::get(url)?;

    // Check if the request was successful (HTTP 200)
    if response.status() != StatusCode::OK {
        eprintln!("Failed to download {}: HTTP {}", url, response.status());
        return Err("Download failed: Non-200 response".into());
    }

    // Check if the response contains an actual image
    if !is_image(&response) {
        eprintln!("Error: The URL does not contain an image.");
        return Err("Invalid content: Not an image".into());
    }

    // Create the output file
    let mut file = File::create(path)?;

    // Copy the response body into the file
    let content = response.bytes()?;
    copy(&mut content.as_ref(), &mut file)?;

    println!("File successfully downloaded to '{}'", output_path);
    Ok(())
}

// Function to check if response is an image
fn is_image(response: &Response) -> bool {
    if let Some(content_type) = response.headers().get("Content-Type") {
        let content_type_str = content_type.to_str().unwrap_or("");
        return content_type_str.starts_with("image/");
    }
    false
}

//⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠

//DANGEROUS!!!! ONLY DO IF ALL REFERENCES ARE DELETED OR WE ARE SURE THE VARIABLE IS NEVER USED AGAIN 

//⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠⚠
//pub unsafe fn reclaim_texture(texture: &Texture2D) {
//    let leaked_ptr: *mut Texture2D = texture as *const Texture2D as *mut Texture2D;
//    unsafe {
//        let _reclaimed_box = Box::from_raw(leaked_ptr); // This will be dropped at the end of scope
//    }
//}


