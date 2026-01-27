//! lilCopy - Seize the memes of distribution
//! 
//! Extracts thumbnails from media files and copies them to clipboard
//! for Discord-friendly sharing.

// Hide console window when launched from Explorer
#![windows_subsystem = "windows"]

use std::env;
use std::path::Path;

use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::SIZE,
        Graphics::Gdi::{
            CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetObjectW, SelectObject,
            BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
        },
        System::Com::{
            CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED,
        },
        UI::Shell::{IShellItemImageFactory, SHCreateItemFromParsingName, SIIGBF_THUMBNAILONLY, SIIGBF_BIGGERSIZEOK},
    },
};

/// Default thumbnail size (Discord-friendly)
const THUMB_SIZE: i32 = 128;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: lilcopy <file_path> [size]");
        eprintln!("  size: thumbnail dimension in pixels (default: {})", THUMB_SIZE);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let thumb_size = args.get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(THUMB_SIZE);

    if let Err(e) = copy_thumbnail(file_path, thumb_size) {
        eprintln!("Error: {}", e);
        // Show a Windows notification on error (optional enhancement)
        std::process::exit(1);
    }
    
    println!("Thumbnail copied to clipboard! 📋");
}

fn copy_thumbnail(file_path: &str, size: i32) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("File not found: {}", file_path).into());
    }

    // Extract thumbnail using Windows Shell API
    let rgba_data = unsafe { extract_thumbnail_windows(file_path, size)? };
    
    // Copy to clipboard using arboard (cross-platform clipboard library)
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_image(rgba_data)?;
    
    Ok(())
}

/// Extract thumbnail using Windows Shell IShellItemImageFactory
unsafe fn extract_thumbnail_windows(
    file_path: &str,
    size: i32,
) -> Result<arboard::ImageData<'static>, Box<dyn std::error::Error>> {
    // Initialize COM - .ok() converts HRESULT to Result
    CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
    
    let result = extract_thumbnail_inner(file_path, size);
    
    CoUninitialize();
    
    result
}

unsafe fn extract_thumbnail_inner(
    file_path: &str,
    size: i32,
) -> Result<arboard::ImageData<'static>, Box<dyn std::error::Error>> {
    // Create shell item from file path
    let wide_path = HSTRING::from(file_path);
    let shell_item: IShellItemImageFactory = SHCreateItemFromParsingName(
        PCWSTR(wide_path.as_ptr()),
        None,
    )?;

    // Get thumbnail bitmap - try thumbnail only first, fall back to any size
    let desired_size = SIZE { cx: size, cy: size };
    let hbitmap = shell_item.GetImage(desired_size, SIIGBF_THUMBNAILONLY)
        .or_else(|_| shell_item.GetImage(desired_size, SIIGBF_BIGGERSIZEOK))?;

    // Convert HBITMAP to RGBA pixel data
    let rgba = hbitmap_to_rgba(hbitmap, size)?;
    
    // Clean up
    DeleteObject(hbitmap);

    Ok(rgba)
}

/// Convert Windows HBITMAP to RGBA pixel data
unsafe fn hbitmap_to_rgba(
    hbitmap: windows::Win32::Graphics::Gdi::HBITMAP,
    _expected_size: i32,  // Not used - we query actual size
) -> Result<arboard::ImageData<'static>, Box<dyn std::error::Error>> {
    // First, get the actual bitmap dimensions
    let mut bitmap_info: BITMAP = std::mem::zeroed();
    let result = GetObjectW(
        hbitmap,
        std::mem::size_of::<BITMAP>() as i32,
        Some(&mut bitmap_info as *mut _ as *mut _),
    );
    
    if result == 0 {
        return Err("Failed to get bitmap info".into());
    }
    
    let width = bitmap_info.bmWidth;
    let height = bitmap_info.bmHeight;
    
    if width <= 0 || height <= 0 {
        return Err(format!("Invalid bitmap dimensions: {}x{}", width, height).into());
    }
    
    let hdc = CreateCompatibleDC(None);
    if hdc.is_invalid() {
        return Err("Failed to create compatible DC".into());
    }

    let old_bitmap = SelectObject(hdc, hbitmap);

    // Set up bitmap info header for 32-bit RGBA
    let mut bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height, // Negative for top-down bitmap
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [Default::default()],
    };

    // Allocate buffer for pixel data
    let pixel_count = (width * height) as usize;
    let mut pixels: Vec<u8> = vec![0; pixel_count * 4];

    // Get the bitmap bits
    let result = GetDIBits(
        hdc,
        hbitmap,
        0,
        height as u32,
        Some(pixels.as_mut_ptr() as *mut _),
        &mut bmi,
        DIB_RGB_COLORS,
    );

    // Clean up GDI objects
    SelectObject(hdc, old_bitmap);
    DeleteDC(hdc);

    if result == 0 {
        return Err("Failed to get bitmap bits".into());
    }

    // Windows returns BGRA, convert to RGBA
    for chunk in pixels.chunks_exact_mut(4) {
        chunk.swap(0, 2); // Swap B and R
    }

    Ok(arboard::ImageData {
        width: width as usize,
        height: height as usize,
        bytes: pixels.into(),
    })
}