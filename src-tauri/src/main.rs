#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use image::DynamicImage;

#[tauri::command(rename_all = "snake_case")]
fn sprite_dropped(payload: &str){
    let mut file_names : Vec<String> =  serde_json::from_str(payload).expect("Error parsing JSON");
    let out_name = {
        let file_name = file_names[0].clone();
        let file_name = file_name.as_str();
        let file_name = file_name.rsplit('.').collect::<Vec<_>>()[1];
        let file_name = file_name.trim_end_matches(|c: char| c.is_ascii_digit());
        let file_name = format!("{}.png", file_name);
        file_name
    };
    file_names.sort();

    let mut imgs: Vec<DynamicImage> = Vec::new();
    for file in file_names {
        imgs.push(image::open(file).unwrap())
    }

    let img_count = imgs.len();
    let in_img_width = imgs[0].width();
    let in_img_height = imgs[0].height();

    let out_img_width = in_img_width * img_count as u32;
    let out_img_height = in_img_height;

    let mut out_buffer = image::ImageBuffer::new(out_img_width, out_img_height);
    for (_, _, pixel) in out_buffer.enumerate_pixels_mut() {
        *pixel = image::Rgba([0, 0, 0, 0]);
    }

    for (i, img) in imgs.into_iter().enumerate() {
        image::imageops::overlay(&mut out_buffer, &img, (in_img_width * i as u32) as i64, 0);
    }
    println!("{}", out_name);
    out_buffer.save(out_name).unwrap()

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sprite_dropped])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
